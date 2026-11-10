# backend

## Purpose

evie's Rust gRPC backend. It owns the domain data; the frontend server calls it, and the browser never does.
For now it's an empty crate: the service, database and migrations come with the [v0.2 milestone](https://github.com/thephilippbusch/evie/milestone/2) issues.

## Structure

| Path          | Contents                                                            |
| ------------- | ------------------------------------------------------------------- |
| `src/main.rs` | Startup and wiring only                                             |
| `src/lib.rs`  | The crate's modules and public API, so integration tests can use it |
| `Cargo.toml`  | Crate manifest; version and edition come from the workspace         |

The workspace files live at the repository root: `Cargo.toml`, `Cargo.lock` and `rust-toolchain.toml`.

## Commands

The tasks come from [`.moon/tasks/rust.yml`](../../.moon/tasks/rust.yml), shared by every Rust crate.

| Command                   | What it does                                    |
| ------------------------- | ----------------------------------------------- |
| `moon run backend:build`  | Compile the crate                               |
| `moon run backend:lint`   | Check formatting with rustfmt, lint with clippy |
| `moon run backend:test`   | Run the unit and integration tests              |
| `moon run backend:format` | Format the crate with rustfmt                   |

## Local rules

- Follow the `rust-conventions` skill.
- Add crates with `cargo add <crate>` inside `apps/backend/`. Commit `Cargo.lock`, since every task runs with `--locked`.
