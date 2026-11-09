# Modules

## Layout

Group by domain feature. A module with children is `name.rs` plus a `name/` folder; never `name/mod.rs`.

```text
src/
├── main.rs          # wiring and startup only
├── lib.rs           # module tree and the crate's public API
├── booking.rs       # declares booking's children, re-exports its API
├── booking/
│   ├── error.rs
│   └── service.rs
├── grpc.rs
└── grpc/
    └── error.rs     # domain error -> tonic::Status
tests/
├── common/
│   └── mod.rs       # shared test helpers; the only mod.rs
└── booking.rs       # integration tests for booking
```

Not this:

```text
src/
├── models/
├── services/
└── booking/
    └── mod.rs
```

## `main.rs` stays thin

`main.rs` reads config, wires dependencies and starts the server. Everything else lives behind `lib.rs`, so integration tests can call it.

```rust
// src/lib.rs
pub mod booking;
mod config;
mod db;
mod grpc;

pub use config::Config;
pub use db::connect;
pub use grpc::serve;
```

## Visibility

Private by default. `pub(crate)` for items other modules in the crate need, `pub` only for the crate's API.

```rust
// src/booking/service.rs
pub struct BookingService {
    db: sea_orm::DatabaseConnection, // private field
}

impl BookingService {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }

    pub(crate) fn db(&self) -> &sea_orm::DatabaseConnection {
        &self.db // used by the grpc module, not by other crates
    }
}
```

## Re-exports

The parent module declares its children privately and re-exports what callers need. Callers write `crate::booking::BookingService`, never `crate::booking::service::BookingService`.

```rust
// src/booking.rs
mod error;
mod service;

pub use error::BookingError;
pub use service::BookingService;
```

Name each item you re-export. No `pub use service::*;`: it hides where names come from and leaks new items by accident.
