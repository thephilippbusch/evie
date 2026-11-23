# evie-dev

## Purpose

The repository's development tool, one subcommand per repo-wide check. It runs the commit hooks, the agent file check and the Claude Code guardrails, so they need only the proto-pinned toolchain. It never runs in production.

## Structure

| Path                | Contents                                                       |
| ------------------- | -------------------------------------------------------------- |
| `src/main.rs`       | Argument parsing and exit codes only                           |
| `src/lib.rs`        | One module per subcommand                                      |
| `src/commit_msg.rs` | `commit-msg`: the Conventional Commit check                    |
| `tests/<module>.rs` | Integration tests per subcommand, with accept and reject cases |
| `Cargo.toml`        | Crate manifest; version and edition come from the workspace    |

The workspace files live at the repository root: `Cargo.toml`, `Cargo.lock` and `rust-toolchain.toml`.

## Commands

The tasks come from [`.moon/tasks/rust.yml`](../../.moon/tasks/rust.yml), shared by every Rust crate.

| Command                    | What it does                                                                      |
| -------------------------- | --------------------------------------------------------------------------------- |
| `moon run evie-dev:build`  | Compile the crate                                                                 |
| `moon run evie-dev:lint`   | Lint with clippy                                                                  |
| `moon run evie-dev:check`  | Run `lint` (clippy also checks the types), then check the formatting with rustfmt |
| `moon run evie-dev:test`   | Run the unit and integration tests                                                |
| `moon run evie-dev:format` | Format the crate with rustfmt                                                     |

## Local rules

- Follow the `rust-conventions` skill.
- Add crates with `cargo add <crate>` inside `tools/evie-dev/`. Commit `Cargo.lock`, since every task runs with `--locked`. Keep the dependencies few: every fresh clone compiles them before its first commit.
- A subcommand keeps the exit codes and messages its callers rely on. Change them only together with the callers.
