# backend

## Purpose

evie's Rust gRPC backend. It owns the domain data; the frontend server calls it, and the browser never does.
For now it serves only the standard gRPC health service (`grpc.health.v1`). The server-wide status (service `""`) is `SERVING` while PostgreSQL answers a ping and `NOT_SERVING` otherwise; it starts as `NOT_SERVING` and is checked every 5 seconds.
The backend starts without a database and never migrates. The [`domain-migrations`](../../libs/domain-migrations/AGENTS.md) crate owns the domain schema.

## Structure

| Path                  | Contents                                                            |
| --------------------- | ------------------------------------------------------------------- |
| `src/main.rs`         | Startup and wiring only: config, logging, signals                   |
| `src/lib.rs`          | The crate's modules and public API, so integration tests can use it |
| `src/config.rs`       | `Config`, read from the environment variables below                 |
| `src/telemetry.rs`    | Structured logging with `tracing`                                   |
| `src/db.rs`           | The SeaORM connection pool, which connects on first use             |
| `src/health.rs`       | Pings the database and sets the health status                       |
| `src/grpc.rs`         | The Tonic server and graceful shutdown                              |
| `tests/health.rs`     | Health checks with a reachable and an unreachable database          |
| `tests/common/mod.rs` | Test helpers: a PostgreSQL container and a server on a free port    |
| `Cargo.toml`          | Crate manifest; version and edition come from the workspace         |

The workspace files live at the repository root: `Cargo.toml`, `Cargo.lock` and `rust-toolchain.toml`.

## Configuration

| Variable       | Default         | Meaning                                                    |
| -------------- | --------------- | ---------------------------------------------------------- |
| `DATABASE_URL` | required        | PostgreSQL URL, e.g. `postgres://user:pass@localhost/evie` |
| `GRPC_ADDR`    | `0.0.0.0:50051` | Address the gRPC server listens on                         |
| `LOG_FORMAT`   | `json`          | `json` for one object per line, `pretty` for local reading |
| `RUST_LOG`     | `info`          | Log filter, e.g. `backend=debug,info`                      |

## Commands

The tasks come from [`.moon/tasks/rust.yml`](../../.moon/tasks/rust.yml), shared by every Rust crate.

| Command                   | What it does                                                                      |
| ------------------------- | --------------------------------------------------------------------------------- |
| `moon run backend:build`  | Compile the crate                                                                 |
| `moon run backend:lint`   | Lint with clippy                                                                  |
| `moon run backend:check`  | Run `lint` (clippy also checks the types), then check the formatting with rustfmt |
| `moon run backend:test`   | Run the unit and integration tests; Docker must run for the PostgreSQL container  |
| `moon run backend:format` | Format the crate with rustfmt                                                     |

## Local rules

- Follow the `rust-conventions` skill.
- Run the backend locally with `cargo run` in `apps/backend/`, with `DATABASE_URL` set. Apply the migrations first with `moon run domain-migrations:migrate`.
- Integration tests start PostgreSQL with `testcontainers`. Use the same image tag as `libs/domain-migrations/tests/migrate.rs`.
- Add crates with `cargo add <crate>` inside `apps/backend/`. Commit `Cargo.lock`, since every task runs with `--locked`.
