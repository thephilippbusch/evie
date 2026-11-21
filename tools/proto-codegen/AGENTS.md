# proto-codegen

## Purpose

A development tool that generates the code in [`libs/proto`](../../libs/proto/AGENTS.md) from its protobuf definitions. It never runs in production.
It writes the Rust code with protox and tonic-prost-build, so no `protoc` is needed, and runs `buf generate` for the TypeScript code. It doesn't depend on the `evie-proto` crate, so it builds even when `libs/proto/gen/` is broken.

## Structure

| Path          | Contents                                                                           |
| ------------- | ---------------------------------------------------------------------------------- |
| `src/main.rs` | The generator: `proto-codegen <root>` writes `<root>/gen/rust` and `<root>/gen/ts` |
| `Cargo.toml`  | Crate manifest; version and edition come from the workspace                        |

## Commands

The tasks come from [`.moon/tasks/rust.yml`](../../.moon/tasks/rust.yml). To generate code, use the tasks in `libs/proto`: `moon run proto:codegen` writes `gen/`, and `moon run proto:check-codegen` compares it with fresh output.

| Command                         | What it does                                                                      |
| ------------------------------- | --------------------------------------------------------------------------------- |
| `moon run proto-codegen:build`  | Compile the tool                                                                  |
| `moon run proto-codegen:lint`   | Lint with clippy                                                                  |
| `moon run proto-codegen:check`  | Run `lint` (clippy also checks the types), then check the formatting with rustfmt |
| `moon run proto-codegen:test`   | Run the tests                                                                     |
| `moon run proto-codegen:format` | Format the crate with rustfmt                                                     |

## Local rules

- Follow the `rust-conventions` skill.
- Run it from `libs/proto`, where `buf.gen.yaml` is, and only through the `proto` tasks, which put buf on the `PATH`.
- Keep the output deterministic: the same definitions must give byte-identical files, or `check-codegen` fails.
- After a change here, run `moon run proto:codegen` and commit any change in `libs/proto/gen/` with it.
