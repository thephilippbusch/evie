---
name: rust-conventions
description: Sets evie's Rust conventions that clippy and rustfmt don't enforce, for errors, modules, async code and tests, each with an example. Use when writing or reviewing Rust code, or adding a crate, module, error type, async task or test.
---

# Rust conventions

Rules clippy and rustfmt don't check. For everything else, follow their output.
Stack-specific patterns (gRPC services, SeaORM entities and migrations) belong to their own skills, such as `grpc-rpc` and `seaorm-migration` (once they exist).

| Read                                           | When you                                                 |
| ---------------------------------------------- | -------------------------------------------------------- |
| [references/errors.md](references/errors.md)   | define, return, log or map an error to a `tonic::Status` |
| [references/modules.md](references/modules.md) | add a file, module or crate, or decide what is `pub`     |
| [references/async.md](references/async.md)     | write async code, spawn a task, take a lock or call out  |
| [references/testing.md](references/testing.md) | add or run a unit or integration test                    |

## Errors

- Each domain module has one `thiserror` enum named for it, e.g. `BookingError`. No `String` errors, no `Box<dyn Error>`.
- A variant names what went wrong; wrap a lower-level error with `#[from]` or `#[source]`.
- Messages are lowercase with no trailing period, and don't repeat the source's message.
- `anyhow` only in `main.rs` and the startup code it calls, with `.context(...)`. Never in a library or handler signature.
- Domain errors become a `tonic::Status` in one `From` impl at the gRPC edge. Domain code never builds a `Status`.
- Internal errors reach the client as `Status::internal` with a generic message; log the details at the edge, once.
- No `unwrap` or `expect` outside tests and startup. The exception is `Mutex::lock`, with `expect("<name> lock poisoned")`.

## Modules

- Use `name.rs` next to a `name/` folder, never `name/mod.rs`. The only `mod.rs` is `tests/common/mod.rs`.
- Group code by domain feature (`booking`, `event`), not by layer (`models`, `services`).
- `main.rs` only wires and starts things; the logic lives in `lib.rs` and its modules, so integration tests can reach it.
- Items are private by default, `pub(crate)` for the rest of the crate, `pub` only for the crate's API.
- A parent module re-exports its children's public items with `pub use`; callers never reach into deep paths. No glob re-exports.

## Async

- Tokio is the only runtime; `#[tokio::main]` in `main.rs`, nowhere else.
- No blocking calls in async code: use `tokio::fs` and `tokio::time::sleep`, and move CPU-heavy or sync work to `spawn_blocking`.
- Use `std::sync::Mutex` for short critical sections and drop the guard before any `.await`. Use `tokio::sync::Mutex` only when the lock must span an `.await`.
- Every `.await` can be the last: a dropped request cancels the handler there. Keep multi-step writes in one transaction, and spawn work that must finish.
- In `tokio::select!`, only race cancel-safe futures, such as `mpsc::Receiver::recv` or `CancellationToken::cancelled`.
- Track spawned tasks in a `JoinSet` or keep the `JoinHandle`; long-running tasks stop on a `CancellationToken`.
- Wrap every outbound call in `tokio::time::timeout`.

## Testing

- Unit tests sit at the bottom of the file in `#[cfg(test)] mod tests` with `use super::*;`.
- Integration tests live in `tests/<feature>.rs` and use only the crate's public API.
- Async tests use `#[tokio::test]`; time-dependent ones use `start_paused = true` instead of real sleeps.
- Name a test for the behaviour it checks, without a `test_` prefix: `book_rejects_a_full_place`.
- Assert on error variants with `matches!`, not on message strings.
- Run tests with `moon run <project>:test`; add `-- <filter>` for one test. Use the `moon-run` skill.
