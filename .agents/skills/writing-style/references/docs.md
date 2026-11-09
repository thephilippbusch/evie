# Docs

`AGENTS.md` files, skills, ADRs and other Markdown in the repository. The `authoring-skills` skill sets their structure and size; this file covers the sentences.

## Outcome first, no meta talk

Before:

> In this section we will take a look at how you can go about running the tests. It's worth noting that there are actually several different ways to do this.

After:

> Run the tests with `moon run <project>:test`.

## Tables and bullets over prose

Before:

> The frontend lives in apps/web and is a SolidStart app, while the backend, which is written in Rust, can be found in apps/backend. Shared protobuf definitions are kept in libs/proto.

After:

> | Path           | Contents                    |
> | -------------- | --------------------------- |
> | `apps/web`     | SolidStart frontend and BFF |
> | `apps/backend` | Rust gRPC backend           |
> | `libs/proto`   | protobuf definitions        |

## No hedging, no filler

Before:

> You should probably try to avoid editing generated files, since this could potentially cause some issues down the line.

After:

> Never edit generated files. Change the source and run `codegen`.

## Link, don't repeat

Before:

> To add a task, open moon.yml, add a key under `tasks` with a `command`, a `type` (build, test or run), `inputs` …

After:

> Add the task with the `moon-add-task` skill.

## Current state, not history

Before:

> We used to run Prettier here, but after some discussion we switched to dprint, which is now used for Markdown, JSON and YAML.

After:

> dprint formats Markdown, JSON and YAML.
