---
name: moon-query
description: Looks up moon projects, their tasks, and the projects and tasks a change affects, through the moon MCP tools or the moon CLI. Use when asked which projects or tasks exist, which project owns a file, or what a change affects.
---

# moon-query

Prefer the moon MCP tools. Fall back to the CLI when they aren't available, or when you work in a different git worktree than the one the agent session started in: the MCP server answers for the session's checkout.

## MCP tools

| Question                                          | Tool                                                       |
| ------------------------------------------------- | ---------------------------------------------------------- |
| All projects, with their `source` directory       | `get_projects`                                             |
| One project: its config and tasks                 | `get_project` with `id`                                    |
| All tasks                                         | `get_tasks`                                                |
| One task: command, inputs, options, type          | `get_task` with `target`, e.g. `root:validate-agent-files` |
| Uncommitted changed files                         | `get_changed_files` with `remote: false`                   |
| Files changed on the branch, compared with `main` | `get_changed_files` with `remote: true`                    |

## CLI

`moon query` prints JSON; filter it with `jq`. The other commands print tables.

| Question                                 | Command                                                             |
| ---------------------------------------- | ------------------------------------------------------------------- |
| All projects or tasks                    | `moon projects`, `moon tasks`                                       |
| One project or task                      | `moon project <id>`, `moon task <project>:<task>`                   |
| Project IDs and sources                  | `moon query projects \| jq -r '.projects[] \| "\(.id) \(.source)"'` |
| Projects by layer or task                | `moon query projects --layer library`, `--tasks build`              |
| Uncommitted changed files                | `moon query changed-files --local`                                  |
| Files changed on the branch              | `moon query changed-files --remote`                                 |
| Projects affected by uncommitted changes | `moon query projects --affected \| jq -r '.projects[].id'`          |
| Projects affected by the branch          | `moon query projects --affected remote \| jq -r '.projects[].id'`   |
| Include the projects that depend on them | add `--downstream deep`                                             |
| Affected tasks, and the file behind each | `moon query affected` (`moon query affected remote` for the branch) |

## Which project owns a file

- Every folder in `apps/` and `libs/` is a project, matched by the globs in `.moon/workspace.yml`. Its ID is the folder name and its `source` is the folder.
- The repository root is the `root` project, with source `.`.
- A file belongs to the project with the deepest `source` that contains it: `apps/web/src/app.tsx` → `web`, `scripts/format-staged.sh` → `root`.
- Since `root` contains every file, moon lists `root` as affected by any change. Only `root` tasks whose `inputs` match a changed file run, such as `root:validate-agent-files` for skill and `AGENTS.md` changes.

## Steps: what does a change affect

1. List the changed files: `get_changed_files` with `remote: false` for uncommitted work, `remote: true` for the whole branch.
2. Map each file to its project by the deepest `source`, or run `moon query projects --affected` (`--affected remote` for the branch).
3. If other projects depend on an affected library, add `--downstream deep` to include them.
4. To see which tasks will run and why, run `moon query affected`.
5. Run their checks with the `moon-run` skill.

## Output

The affected project IDs and, if asked, their affected tasks as `<project>:<task>` targets.
