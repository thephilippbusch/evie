---
name: moon-run
description: Runs moon tasks, such as one task, a task in every project, the checks of affected projects, or what CI runs. Use when building, linting, formatting, testing or checking code, or when asked to run a task or run what CI runs.
---

# moon-run

Run tasks through moon, not by calling `bun`, `cargo` or a script directly. moon runs the task's dependencies first, reuses cached results, and uses the toolchain pinned by proto. Only adding dependencies (`bun add`, `cargo add`) goes to the native tool.

## Commands

| Goal                                          | Command                                                                |
| --------------------------------------------- | ---------------------------------------------------------------------- |
| One task                                      | `moon run <project>:<task>`, e.g. `moon run root:validate-agent-files` |
| A task in every project that has it           | `moon run :<task>`, e.g. `moon run :lint`                              |
| Several tasks                                 | `moon run :lint :test`                                                 |
| Only tasks affected by uncommitted changes    | `moon run :lint :test --affected`                                      |
| Only tasks affected by the branch, vs. `main` | `moon run :lint :test --affected remote`                               |
| Also the projects that depend on them         | add `--downstream deep`                                                |
| The build and test tasks of one project       | `moon check <project>`                                                 |
| The build and test tasks a change affects     | `moon check --all --affected`                                          |
| ... affected by the branch, vs. `main`        | `moon check --all --affected remote`                                   |
| Everything CI runs for this branch            | `moon ci`                                                              |
| Pass arguments to the underlying command      | `moon run <project>:<task> -- <args>`                                  |
| Ignore the cache and the affected check       | add `--force`                                                          |
| See a task's output live                      | add `--output-style stream`                                            |

## Steps: check a change

1. Find the affected projects with the `moon-query` skill, if you need the list.
2. Run `moon check --all --affected` for uncommitted work, or `moon check --all --affected remote` for the whole branch.
3. Before a PR, run `moon ci`. It compares the branch with `main` and runs exactly the tasks CI runs.
4. On a failure, rerun the failing target alone: `moon run <project>:<task> --output-style stream`.

The `verify` skill (once it exists) wraps these steps.

## How moon decides what runs

- **Dependencies:** a task's `deps` run first, in graph order: `moon run web:build` also runs a `^:test` or `~:codegen` dep.
- **Cache:** moon hashes the command, args, env and `inputs`. On a match it replays the last result and prints `cached`. Tasks with `cache: false` always run.
- **Affected:** with `--affected`, a task runs only if a changed file matches its `inputs`. A task with `inputs: []` is never affected.
- **Task types:** `moon check` and `moon ci` run `build` and `test` tasks and skip `run` tasks, such as dev servers, formatters and the commit hooks.
- **CI:** `moon ci` runs the affected tasks with the default `runInCI`, skips `runInCI: false`, and always runs `runInCI: 'always'`.
- **Outputs:** a `build` task fails with `missing_outputs` if it doesn't create every path in its `outputs`.

## Rules

- Never run `root:format-staged` or `root:lint-commit-msg` by hand; the git hooks run them.
