# proto

## Purpose

The protobuf definitions of evie's gRPC API. The backend serves it, and the frontend server calls it; the browser never does.

## Structure

| Path           | Contents                                                                    |
| -------------- | --------------------------------------------------------------------------- |
| `definitions/` | The `.proto` files, one folder per package: `evie/<area>/v<n>/<area>.proto` |
| `buf.yaml`     | The buf module and its lint rules                                           |
| `package.json` | The bun package `@evie/proto`, with buf as a dev dependency                 |

## Commands

| Command                     | What it does                                     |
| --------------------------- | ------------------------------------------------ |
| `moon run proto:lint-proto` | Lint the definitions with buf's `STANDARD` rules |

## Local rules

- Follow buf's [`STANDARD`](https://buf.build/docs/lint/rules/) style: versioned packages such as `evie.ping.v1`, folders that match the package, services named `…Service`, and `…Request` and `…Response` messages for each RPC.
- Never reuse or renumber a field. Mark a removed field's number and name `reserved`.
