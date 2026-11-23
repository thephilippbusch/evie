# evie-dev

## Purpose

The repository's development tool, one subcommand per repo-wide check. It runs the commit hooks, the agent file check and the Claude Code guardrails, so they need only the proto-pinned toolchain. It never runs in production.

## Structure

| Path                   | Contents                                                                                           |
| ---------------------- | -------------------------------------------------------------------------------------------------- |
| `bin/evie-dev`         | The launcher: builds the release binary when it is missing or older than the sources, then runs it |
| `src/main.rs`          | Argument parsing and exit codes only                                                               |
| `src/lib.rs`           | One module per subcommand                                                                          |
| `src/commit_msg.rs`    | `commit-msg`: the Conventional Commit check                                                        |
| `src/format_staged.rs` | `format-staged`: formats the staged files with dprint                                              |
| `src/agent_files.rs`   | `validate-agent-files`: the rules of the Budgets table in the `authoring-skills` skill             |
| `src/agent_files/`     | The frontmatter readers, the Agent Skills spec rules and the Markdown link finder                  |
| `src/pre_tool_use.rs`  | `pre-tool-use`: the Claude Code guardrails for one tool call                                       |
| `src/pre_tool_use/`    | The Bash command reader, the protected-path rules and shell glob matching                          |
| `tests/common/mod.rs`  | Test helpers, such as a temporary git repository                                                   |
| `tests/<module>.rs`    | Integration tests per subcommand, with accept and reject cases                                     |
| `Cargo.toml`           | Crate manifest; version and edition come from the workspace                                        |

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

## Callers

| Subcommand             | Called by                                                                           |
| ---------------------- | ----------------------------------------------------------------------------------- |
| `commit-msg`           | `root:lint-commit-msg` (the `commit-msg` hook) and `.github/workflows/pr-title.yml` |
| `format-staged`        | `root:format-staged` (the `pre-commit` hook)                                        |
| `validate-agent-files` | `root:validate-agent-files`, which CI runs on every PR                              |
| `pre-tool-use`         | both PreToolUse hooks in `.claude/settings.json`; exit 2 blocks the tool call       |

Hooks and moon tasks call `tools/evie-dev/bin/evie-dev <subcommand>`, never `cargo run`, so only the first call after a change compiles. Only the PR title workflow runs `cargo run --locked -p evie-dev`, since it calls the tool once. Try the guardrails by piping a tool call into it:

```sh
echo '{"tool_name":"Bash","tool_input":{"command":"git push --force"}}' | tools/evie-dev/bin/evie-dev pre-tool-use; echo "exit $?"
```

## Local rules

- Follow the `rust-conventions` skill.
- Add crates with `cargo add <crate>` inside `tools/evie-dev/`. Commit `Cargo.lock`, since every task runs with `--locked`. Keep the dependencies few: every fresh clone compiles them before its first commit.
- A subcommand keeps the exit codes and messages its callers rely on. Change them only together with the callers.
