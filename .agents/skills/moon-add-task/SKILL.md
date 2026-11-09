---
name: moon-add-task
description: Adds a task to a project's moon.yml or the root moon.yml, following evie's task conventions. Use when adding or changing a moon task, a build, lint, format, test, codegen or migrate command, or a repo-wide script.
---

# moon-add-task

The root `moon.yml` is the style reference: read it before you add a task.

## Steps

1. Pick the file:
   - `<project>/moon.yml` for a task of one app or library.
   - The root `moon.yml` for a repo-wide task. It runs from the repository root, as `root:<task>`, and its script goes in `scripts/`.
   - A file in `.moon/tasks/` for a task that every project of a kind needs. See [Inherited tasks](#inherited-tasks).
2. Name the task. Use a standard name where it fits, so `moon run :<task>` reaches every project: `build`, `lint`, `format`, `test`, `codegen`, `migrate`. Otherwise use a kebab-case name that starts with a verb, like `validate-agent-files`.
3. Write the task with the keys below, in this order. Add a YAML comment for every choice that isn't obvious, such as why a task isn't cached.
4. Check the resolved task: `moon task <project>:<task>`.
5. Run it with the `moon-run` skill: `moon run <project>:<task>`.
6. Add it to the Commands section of the project's `AGENTS.md`, then run `moon run root:validate-agent-files`.

## Keys

```yaml
tasks:
  test:
    description: 'Run the unit tests.'
    command: 'cargo test'
    type: 'test'
    deps:
      - '~:codegen'
    inputs:
      - 'src/**/*'
      - 'Cargo.toml'
```

| Key           | Rule                                                                                                                                                                                                                                                             |
| ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description` | One sentence on what it does. Name who runs it if that isn't a person, e.g. "Run by the pre-commit hook."                                                                                                                                                        |
| `command`     | The command line. A root script runs as `sh scripts/<name>.sh`.                                                                                                                                                                                                  |
| `type`        | `build` if it creates `outputs`; `test` for checks such as lint, typecheck, tests and validation; `run` for servers, hooks and tasks that change files, such as `format` and `migrate`. `moon check` and `moon ci` skip `run` tasks.                             |
| `deps`        | Tasks that must run first: `~:<task>` in the same project, `^:<task>` in every project listed in `dependsOn`, `<project>:<task>` for one project.                                                                                                                |
| `inputs`      | The files whose changes should rerun the task, relative to the project; `/` starts a repo-root path. Always set them: without `inputs`, every file in the project counts. `inputs: []` means the task is never affected, only for tasks run by hand or by hooks. |
| `outputs`     | For `build` tasks: every path the task creates. moon caches them and fails the task if one is missing.                                                                                                                                                           |
| `options`     | Only when a default is wrong; see below.                                                                                                                                                                                                                         |

| Option        | Default    | Set it when                                                                                                    |
| ------------- | ---------- | -------------------------------------------------------------------------------------------------------------- |
| `cache`       | `true`     | `false` if the result depends on files the inputs can't list, or on the network. Say why in a comment.         |
| `runInCI`     | `affected` | `false` for local-only tasks, such as hooks and formatters. `'always'` if CI must run it on every PR; say why. |
| `outputStyle` | buffered   | `'stream'` when a person watches the output live, e.g. a hook.                                                 |
| `persistent`  | `false`    | `true` for a long-running process, such as a dev server.                                                       |

## Inherited tasks

- A file in `.moon/tasks/` defines tasks that every project matching its `inheritedBy` gets, by `layer`, `language`, `stack` or `tags`. None exist yet. A language match needs `language` set in the project's `moon.yml`.

  ```yaml
  # .moon/tasks/library.yml
  $schema: 'https://moonrepo.dev/schemas/v2/tasks.json'

  inheritedBy:
    layer: 'library'

  tasks:
    lint: ...
  ```

- A project drops or renames an inherited task under `workspace.inheritedTasks` with `exclude` or `rename`. The root `moon.yml` opts out of all of them with `include: []`.
- `moon project <id>` lists the files a project inherits from under "Inherits from".
