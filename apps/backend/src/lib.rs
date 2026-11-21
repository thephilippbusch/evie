//! evie's gRPC backend. It serves the gRPC health service, which reports
//! whether PostgreSQL is reachable. `main.rs` only reads the config and wires
//! these parts together.
mod config;
mod db;
mod grpc;
mod health;
mod telemetry;

pub use config::{Config, ConfigError, LogFormat};
pub use db::connect;
pub use grpc::{ServeError, serve};
pub use health::HealthSettings;
pub use telemetry::init_tracing;
