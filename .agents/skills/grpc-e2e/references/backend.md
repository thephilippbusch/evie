# Step 3: the backend handler

Read `apps/backend/AGENTS.md` and follow the `rust-conventions` skill. The handler is the gRPC edge: it turns the request into domain types, calls the domain service and turns the result into the response. The domain logic and its error enum live in the feature module; with no domain logic, as for ping, the handler is all there is.

## Files

| File                                 | Holds                                                                  |
| ------------------------------------ | ---------------------------------------------------------------------- |
| `src/<feature>.rs`, `src/<feature>/` | the domain service, its `thiserror` enum and, with a table, its entity |
| `src/grpc/<area>.rs`                 | `<Area>Api`, which implements the generated `<Area>Service` trait      |
| `src/grpc/error.rs`                  | one `impl From<<Feature>Error> for Status` per domain error            |
| `src/grpc.rs`                        | `mod` lines for its children, and `add_service` in `serve`             |
| `tests/<area>.rs`                    | integration tests through the generated client                         |
| `tests/common/mod.rs`                | `TestServer::channel`, so tests can build any client                   |

## First RPC

1. In `apps/backend/`: `cargo add --path ../../libs/proto`. The crate is `evie-proto`, used as `evie_proto`.
2. Add `'proto'` to `dependsOn` in `apps/backend/moon.yml`, so a change to the definitions reruns the backend's checks.
3. Turn `src/grpc.rs` into a parent module: one `mod <area>;` per service, with the files in `src/grpc/`. The first domain error adds `mod error;` and `src/grpc/error.rs`.
4. Give `TestServer` in `tests/common/mod.rs` a `channel: Channel` field, cloned from the one it connects, and:

   ```rust
   /// A channel to the server, for the client of the service under test.
   /// Drop every client built on it before `stop`.
   pub fn channel(&self) -> Channel {
       self.channel.clone()
   }
   ```

   In `stop`, drop `self.channel` next to `self.client`, so the shutdown doesn't wait for an open connection.
   With a second test file, each file leaves some helpers unused, and clippy fails on dead code: start `tests/common/mod.rs` with `#![allow(dead_code)]`.
5. Update the Purpose and Structure of `apps/backend/AGENTS.md`: the backend now serves more than the health service.

## Handler

Implement the trait through its module path, so its name can't clash with the domain service:

```rust
// src/grpc/ping.rs
use evie_proto::evie::ping::v1::{PingRequest, PingResponse, ping_service_server};
use tonic::{Request, Response, Status};

/// Serves `evie.ping.v1.PingService`.
pub(crate) struct PingApi;

#[tonic::async_trait]
impl ping_service_server::PingService for PingApi {
    async fn ping(&self, _request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        Ok(Response::new(PingResponse {
            version: env!("CARGO_PKG_VERSION").to_owned(),
        }))
    }
}
```

With a domain service, the handler checks the input, calls the service and lets `?` map its error. The example leaves out who calls: once #34 (_planned_) passes the verified user from the BFF, a user-scoped handler also reads that user and lets the service return `permission_denied` or `not_found` when the user has no access.

```rust
// src/grpc/place.rs
use evie_proto::evie::place::v1::{self as proto, place_service_server};
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::place::PlaceService;

/// Serves `evie.place.v1.PlaceService`.
pub(crate) struct PlaceApi {
    places: PlaceService,
}

impl PlaceApi {
    pub(crate) fn new(places: PlaceService) -> Self {
        Self { places }
    }
}

#[tonic::async_trait]
impl place_service_server::PlaceService for PlaceApi {
    async fn get_place(
        &self,
        request: Request<proto::GetPlaceRequest>,
    ) -> Result<Response<proto::GetPlaceResponse>, Status> {
        // Request validation at the edge builds its own Status; domain errors
        // go through the `From` impls in error.rs.
        let place_id = Uuid::parse_str(&request.get_ref().place_id)
            .map_err(|_| Status::invalid_argument("place_id is not a UUID"))?;

        let place = self.places.get(place_id).await?;

        Ok(Response::new(proto::GetPlaceResponse {
            place: Some(proto::Place {
                place_id: place.id.to_string(),
                name: place.name,
                capacity: place.capacity,
            }),
        }))
    }
}
```

## Errors to status codes

`src/grpc/error.rs` holds the `From` impls, in the shape the `rust-conventions` skill shows. Pick the code by what the client can do about it; the BFF turns each code it handles into a tagged error.

| Domain situation                       | `tonic::Status`                      |
| -------------------------------------- | ------------------------------------ |
| malformed input, such as a bad UUID    | `invalid_argument`                   |
| the thing doesn't exist                | `not_found`                          |
| it exists already                      | `already_exists`                     |
| a rule forbids it now, e.g. place full | `failed_precondition`                |
| the caller may not do it               | `permission_denied`                  |
| database or other internal failure     | `internal("internal error")`, logged |

## Registration

`serve` in `src/grpc.rs` adds each service next to the health service. `db` moves into the health watcher, so clone it for the services first:

```rust
use evie_proto::evie::{ping::v1::ping_service_server::PingServiceServer, place::v1::place_service_server::PlaceServiceServer};

use crate::place::PlaceService;

// in `serve`, before `tokio::spawn(health::watch_database(db, …))`
let places = PlaceService::new(db.clone());

// in the builder
Server::builder()
    .add_service(health_service)
    .add_service(PingServiceServer::new(ping::PingApi))
    .add_service(PlaceServiceServer::new(place::PlaceApi::new(places)))
```

## Integration test

One file per area, through the generated client on the test server's channel:

```rust
// tests/ping.rs
mod common;

use common::TestServer;
use evie_proto::evie::ping::v1::{PingRequest, ping_service_client::PingServiceClient};

#[tokio::test]
async fn ping_returns_the_backend_version() {
    let database_url = common::unreachable_database_url().await;
    let server = TestServer::start(&database_url).await;
    let mut client = PingServiceClient::new(server.channel());

    let response = client.ping(PingRequest {}).await.unwrap().into_inner();

    assert_eq!(response.version, env!("CARGO_PKG_VERSION"));
    drop(client);
    server.stop().await;
}
```

- An RPC that needs no database runs against `unreachable_database_url`, which needs no Docker.
- An RPC that reads tables starts a migrated database with `start_migrated_postgres`, which the `seaorm-migration` skill adds to `tests/common/mod.rs` with the first table.
- Test each status code the RPC's comment names, and assert on the code, not the message:

```rust
// tests/place.rs
mod common;

use common::TestServer;
use evie_proto::evie::place::v1::{GetPlaceRequest, place_service_client::PlaceServiceClient};
use uuid::Uuid;

#[tokio::test]
async fn get_place_returns_not_found_for_an_unknown_id() {
    let (_postgres, database_url) = common::start_migrated_postgres().await;
    let server = TestServer::start(&database_url).await;
    let mut client = PlaceServiceClient::new(server.channel());

    let err = client
        .get_place(GetPlaceRequest { place_id: Uuid::new_v4().to_string() })
        .await
        .unwrap_err();

    assert_eq!(err.code(), tonic::Code::NotFound);
    drop(client);
    server.stop().await;
}
```

Run with `moon run backend:test`, or one test with `moon run backend:test -- ping_returns`. Docker must run for the PostgreSQL container.
