---
name: moon-new-library
description: Adds a library to libs/ or a development tool to tools/ as a moon project with its moon.yml, tasks and AGENTS.md. Use when creating a new library, crate, package, shared code or development tool, such as a code generator, that is never deployed on its own.
---

# moon-new-library

A library is code that apps use, such as protobuf definitions or a shared crate or package. A tool is a development tool for apps and libraries, such as a code generator, that never runs in production. Neither is deployed. For something that ships as its own Docker image, use the `moon-new-app` skill.

The steps are the same for both. A library goes in `libs/<name>/` with `layer: 'library'`, a tool in `tools/<name>/` with `layer: 'tool'`. moon lets a library depend only on the `library`, `scaffolding` and `configuration` layers, so a code generator that a library lists in `dependsOn` takes `layer: 'scaffolding'`, like `tools/proto-codegen`.

Until moon templates for new projects exist (issue #27), follow these steps by hand.

## Steps

1. Pick a short kebab-case name. It becomes the project ID. Check it's free: `moon query projects --id <name>` returns no projects.
2. Create `libs/<name>/`, or `tools/<name>/` for a tool. The `libs/*` and `tools/*` globs in `.moon/workspace.yml` make it a project; there's nothing to register.
3. Set up the language's files, such as `Cargo.toml` or `package.json`, as the `rust-conventions` or `typescript-conventions` skill says. A Rust crate takes `version.workspace = true` and `edition.workspace = true`, joins `members` in the root `Cargo.toml`, and adds its `Cargo.lock` entry to `extra-files` in `release-please-config.json`, as `.agents/skills/moon-new-app/references/release-version.md` shows. A TypeScript project joins the bun workspace by itself; run `bun install` at the root.
4. Write `moon.yml` in the project folder:

   ```yaml
   $schema: 'https://moonrepo.dev/schemas/v2/project.json'

   layer: 'library' # or 'tool', or 'scaffolding' for a generator a library uses
   language: 'rust' # or 'typescript'
   project:
     title: '<name>'
     description: '<One sentence: what the library provides and who uses it.>'

   # Projects this one uses, so affected checks reach their dependents.
   dependsOn: []

   tasks: {}
   ```

5. With `language: 'rust'` or `'typescript'`, the project inherits its standard tasks from `.moon/tasks/` (see the `moon-add-task` skill). Add only what's missing with the `moon-add-task` skill, such as a TypeScript project's `build`, or `codegen` and `migrate`.
6. Write `AGENTS.md` in the project folder with the sections Purpose, Structure, Commands and Local rules, as the `authoring-skills` skill describes. Then add the symlink beside it: `ln -s AGENTS.md <folder>/CLAUDE.md`.
7. Check the result:
   - `moon project <name>` shows the source `libs/<name>` or `tools/<name>`, the layer and the tasks.
   - `moon check <name>` passes.
   - `moon run root:validate-agent-files` passes.

## Output

A new project in `libs/<name>/` or `tools/<name>/` with `moon.yml`, `AGENTS.md` and a `CLAUDE.md` symlink, listed by `moon projects`.
