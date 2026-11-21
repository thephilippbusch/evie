# proto

## Purpose

The protobuf definitions of evie's gRPC API, and the code generated from them. The backend serves the API, and the frontend server calls it; the browser never does.
The Rust crate `evie-proto` holds the messages and the tonic server and client of each service. The backend adds it as a path dependency.
The bun package `@evie/proto` holds the messages and service descriptors in TypeScript, for the frontend server: `import { PingService } from "@evie/proto/evie/ping/v1/ping_pb"`.

## Structure

| Path                 | Contents                                                                              |
| -------------------- | ------------------------------------------------------------------------------------- |
| `definitions/`       | The `.proto` files, one folder per package: `evie/<area>/v<n>/<area>.proto`           |
| `gen/rust/`          | Generated Rust code, one file per package, with `mod.rs` nesting them; committed      |
| `gen/ts/`            | Generated TypeScript code, one `<file>_pb.ts` per `.proto` file; committed            |
| `src/lib.rs`         | The `evie-proto` crate: includes `gen/rust/mod.rs`, e.g. `evie_proto::evie::ping::v1` |
| `src/bin/codegen.rs` | The Rust generator, built only with the `codegen` feature                             |
| `buf.yaml`           | The buf module and its lint rules                                                     |
| `buf.gen.yaml`       | The TypeScript generation with protoc-gen-es                                          |
| `tsconfig.json`      | Compiler options for the `typecheck` task                                             |
| `Cargo.toml`         | Crate manifest; version and edition come from the workspace                           |
| `package.json`       | The bun package `@evie/proto`, with buf and protoc-gen-es as dev dependencies         |

## Commands

`codegen`, `lint-proto`, `typecheck` and the `lint` and `check` changes are in [`moon.yml`](moon.yml); the other tasks come from [`.moon/tasks/rust.yml`](../../.moon/tasks/rust.yml).

| Command                     | What it does                                                                       |
| --------------------------- | ---------------------------------------------------------------------------------- |
| `moon run proto:codegen`    | Regenerate `gen/rust/` with the `codegen` binary and `gen/ts/` with `buf generate` |
| `moon run proto:lint-proto` | Lint the definitions with buf's `STANDARD` rules                                   |
| `moon run proto:lint`       | Run `lint-proto`, then lint the crate and the generator with clippy                |
| `moon run proto:typecheck`  | Check the types of `gen/ts/` with `tsc`                                            |
| `moon run proto:check`      | Run `lint` and `typecheck`, then check the formatting with rustfmt                 |
| `moon run proto:build`      | Compile the crate                                                                  |
| `moon run proto:test`       | Run the crate's tests                                                              |
| `moon run proto:format`     | Format the crate with rustfmt; the generated code isn't formatted or lint-checked  |

## Local rules

- Edit only `definitions/`, then run `moon run proto:codegen` and commit the output with the change. Never hand-edit `gen/`.
- Follow buf's [`STANDARD`](https://buf.build/docs/lint/rules/) style: versioned packages such as `evie.ping.v1`, folders that match the package, services named `…Service`, and `…Request` and `…Response` messages for each RPC.
- Never reuse or renumber a field. Mark a removed field's number and name `reserved`.
