# Errors

## One `thiserror` enum per domain module

Name it for the module and give each failure its own variant. Wrap lower-level errors with `#[from]` (converts with `?`) or `#[source]` (converts by hand).

```rust
// src/booking/error.rs
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum BookingError {
    #[error("place {place_id} is full")]
    PlaceFull { place_id: Uuid },

    #[error("booking {0} not found")]
    NotFound(Uuid),

    #[error("database error")]
    Database(#[from] sea_orm::DbErr),
}
```

Not this:

```rust
pub async fn book(place_id: Uuid) -> Result<Booking, String> {
    Err(format!("place {place_id} is full"))
}
```

## Messages

Lowercase, no trailing period, and no copy of the source's message. Reporters walk the `source()` chain, so `{0}` in the message would print it twice.

```rust
#[error("database error")]         // right
Database(#[from] sea_orm::DbErr),

#[error("Database error: {0}.")]   // wrong: capitalised, period, repeats the source
Database(#[from] sea_orm::DbErr),
```

## `anyhow` only at startup

`main.rs` and the startup code it calls may return `anyhow::Result` and add `.context(...)`. Everything a request runs through returns a typed error, so the gRPC edge can map it.

```rust
// src/main.rs
use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = backend::Config::from_env().context("reading the config")?;
    let db = backend::connect(&config.database_url)
        .await
        .context("connecting to PostgreSQL")?;
    backend::serve(config, db).await.context("serving gRPC")
}
```

## Map to `tonic::Status` in one place

One `From` impl per domain error, in the gRPC module. Handlers then use `?`. Pick the status code by what the client can do about it.

```rust
// src/grpc/error.rs
use tonic::Status;

use crate::booking::BookingError;

impl From<BookingError> for Status {
    fn from(err: BookingError) -> Self {
        match &err {
            BookingError::PlaceFull { .. } => Status::failed_precondition(err.to_string()),
            BookingError::NotFound(_) => Status::not_found(err.to_string()),
            BookingError::Database(_) => {
                tracing::error!(error = ?err, "booking failed");
                Status::internal("internal error")
            }
        }
    }
}
```

```rust
// in a handler
let booking = self.bookings.book(place_id, guest_id).await?; // BookingError -> Status
```

Internal details (SQL, hostnames, stack traces) stay in the log. Log an error once, where it is handled, not at every `?`.

## No `unwrap` or `expect` outside tests and startup

Return an error instead. At startup, prefer `.context(...)?`. The one exception is a poisoned `Mutex`, which means another thread already panicked:

```rust
let mut hits = self.hits.lock().expect("rate limiter lock poisoned");
```

Not this:

```rust
let place = places.iter().find(|p| p.id == place_id).unwrap();
```

But this:

```rust
let place = places
    .iter()
    .find(|p| p.id == place_id)
    .ok_or(BookingError::NotFound(place_id))?;
```
