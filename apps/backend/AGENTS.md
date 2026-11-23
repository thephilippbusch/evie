# backend

## Purpose

evie's Rust gRPC backend. It owns the domain data; the frontend server calls it, and the browser never does.
It serves evie's API, defined in [`proto`](../../libs/proto/AGENTS.md) and used through the `evie-proto` crate. For now that is `evie.ping.v1.PingService`, which returns the backend's version.
It also serves the standard gRPC health service (`grpc.health.v1`). The server-wide status (service `""`) is `SERVING` while PostgreSQL answers a ping and `NOT_SERVING` otherwise; it starts as `NOT_SERVING` and is checked every 5 seconds.
The backend starts without a database and never migrates. The [`domain-migrations`](../../libs/domain-migrations/AGENTS.md) crate owns the domain schema. It is in the backend's `dependsOn`, since the backend image ships the migrations as its `migrate` target; the first SeaORM entities also add it to `Cargo.toml` as a dev dependency, for the integration tests.

## Structure

| Path                  | Contents                                                            |
| --------------------- | ------------------------------------------------------------------- |
| `src/main.rs`         | Startup and wiring only: config, logging, signals                   |
| `src/lib.rs`          | The crate's modules and public API, so integration tests can use it |
| `src/config.rs`       | `Config`, read from the environment variables below                 |
| `src/telemetry.rs`    | Structured logging with `tracing`                                   |
| `src/db.rs`           | The SeaORM connection pool, which connects on first use             |
| `src/health.rs`       | Pings the database and sets the health status                       |
| `src/grpc.rs`         | The Tonic server, the services it serves, and graceful shutdown     |
| `src/grpc/<area>.rs`  | One gRPC service each, e.g. `ping.rs` with `PingApi`                |
| `tests/health.rs`     | Health checks with a reachable and an unreachable database          |
| `tests/ping.rs`       | The ping RPC returns the backend's version                          |
| `tests/common/mod.rs` | Test helpers: a PostgreSQL container and a server on a free port    |
| `Cargo.toml`          | Crate manifest; version and edition come from the workspace         |

The workspace files live at the repository root: `Cargo.toml`, `Cargo.lock` and `rust-toolchain.toml`.

## Configuration

| Variable       | Default           | Meaning                                                               |
| -------------- | ----------------- | --------------------------------------------------------------------- |
| `DATABASE_URL` | required          | PostgreSQL URL, e.g. `postgres://user:pass@localhost/evie`            |
| `GRPC_ADDR`    | `127.0.0.1:50051` | Address the gRPC server listens on; containers set `0.0.0.0:50051`    |
| `LOG_FORMAT`   | `json`            | `json` for one object per line, `pretty` for local reading            |
| `RUST_LOG`     | `info`            | Log filter, e.g. `backend=debug,info`; an invalid one stops the start |

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
- Add or change an RPC end to end with the `grpc-e2e` skill.
- Run the backend locally with `cargo run` in `apps/backend/`, with `DATABASE_URL` set. Apply the migrations first with `moon run domain-migrations:migrate`.
- Or run the whole stack with `docker compose up` at the repository root. Its `migrate` service applies the migrations, then the backend starts on the stack's network only, with no host port.
- The root `Dockerfile` builds the images: `docker build --target backend .` for the backend, `--target migrate` for the one-shot migrations, both from the repository root. They are built from a moon scaffold: keep `dependsOn` in `moon.yml` complete, since `moon docker scaffold` copies only the sources of the backend and its `dependsOn`.
- Integration tests start PostgreSQL with `testcontainers`. Use the same image tag as `libs/domain-migrations/tests/migrate.rs`.
- Add crates with `cargo add <crate>` inside `apps/backend/`. Commit `Cargo.lock`, since every task runs with `--locked`.
