# Step 1: the protobuf definition

Read `libs/proto/AGENTS.md` first. The definitions follow buf's `STANDARD` lint rules, which `moon run proto:lint-proto` checks.

## Where

One file per package: `libs/proto/definitions/evie/<area>/v<n>/<area>.proto`, with `package evie.<area>.v<n>;`. An area is a domain feature, like `ping`, `event` or `booking`. A new RPC on an existing area goes into its file and its service.

## Shape

`libs/proto/definitions/evie/ping/v1/ping.proto` is the model:

```proto
syntax = "proto3";

package evie.place.v1;

// Reads and changes the places that hosts offer.
service PlaceService {
  // Returns one place. NOT_FOUND if it doesn't exist.
  rpc GetPlace(GetPlaceRequest) returns (GetPlaceResponse);
}

message GetPlaceRequest {
  // The place's UUID.
  string place_id = 1;
}

message GetPlaceResponse {
  Place place = 1;
}

message Place {
  // The place's UUID.
  string place_id = 1;
  // The name the host gave the place, such as "Guest room".
  string name = 2;
  // How many people can sleep there.
  int32 capacity = 3;
}
```

- One service per area, named `<Area>Service`. Each RPC has its own `<Rpc>Request` and `<Rpc>Response`, even when they are empty or alike.
- A comment on every service, RPC and field. The RPC comment names the status codes the client should handle.
- Field names are snake_case; IDs are UUID strings named `<thing>_id`.
- proto3 has no required fields: a missing string arrives as `""`, a missing message as unset. The backend checks the input; the BFF decodes the output.
- A well-known type such as `google.protobuf.Timestamp` needs the `prost-types` crate in `libs/proto`: `cargo add prost-types` in `libs/proto/`.

## Generate and check

```sh
moon run proto:codegen   # rewrites libs/proto/gen/rust and libs/proto/gen/ts
moon run proto:check     # buf lint, clippy, tsc on gen/ts, and gen/ matches the definitions
```

Commit the definition and everything `codegen` changed in `gen/` together. `check-codegen` fails in CI when they differ.

## What the code generator makes

For `evie.place.v1`:

| Side       | Import                                                                                                    |
| ---------- | --------------------------------------------------------------------------------------------------------- |
| Rust       | `evie_proto::evie::place::v1::{GetPlaceRequest, GetPlaceResponse, Place}`                                 |
| Rust trait | `evie_proto::evie::place::v1::place_service_server::{PlaceService, PlaceServiceServer}`                   |
| Rust test  | `evie_proto::evie::place::v1::place_service_client::PlaceServiceClient`                                   |
| TypeScript | `import { PlaceService } from "@evie/proto/evie/place/v1/place_pb";` (the service descriptor for Connect) |

## Changing a released message

- Add fields with new numbers; old clients ignore them.
- Never reuse or renumber a field number. When you remove a field, mark its number and name `reserved`:

```proto
message Place {
  reserved 4;
  reserved "description";
}
```

- A change that breaks callers, such as a new meaning for a field, goes into a new version: `evie.place.v2`.
