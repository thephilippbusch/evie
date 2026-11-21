//! The Rust code generated from evie's protobuf definitions: the messages,
//! and the tonic server and client of each service.
//!
//! Each protobuf package is a module, e.g. `evie_proto::evie::ping::v1`.
//! The code lives in `gen/rust/`, written by the `codegen` moon task.

// Generated code: lints are the generator's concern, and rustfmt doesn't
// follow `include!`, so the files aren't format-checked either.
#[allow(clippy::all)]
mod generated {
    include!("../gen/rust/mod.rs");
}

pub use generated::evie;
