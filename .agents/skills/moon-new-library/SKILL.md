---
name: moon-new-library
description: Adds a library to libs/ as a moon project with its moon.yml, tasks and AGENTS.md. Use when creating a new library, crate, package or shared code that apps use but that is never deployed on its own.
---

# moon-new-library

A library is code that apps use, such as protobuf definitions or a shared crate or package. It's never deployed. For something that ships as its own Docker image, use the `moon-new-app` skill.

Until moon templates for new projects exist (issue #27), follow these steps by hand.

## Steps

1. Pick a short kebab-case name. It becomes the project ID. Check it's free: `moon query projects --id <name>` returns no projects.
2. Create `libs/<name>/`. The `libs/*` glob in `.moon/workspace.yml` makes it a project; there's nothing to register.
3. Set up the language's files, such as `Cargo.toml` or `package.json`, as the `rust-conventions` or `typescript-conventions` skill says.
4. Write `libs/<name>/moon.yml`:

   ```yaml
   $schema: 'https://moonrepo.dev/schemas/v2/project.json'

   layer: 'library'
   language: 'rust' # or 'typescript'
   project:
     title: '<name>'
     description: '<One sentence: what the library provides and who uses it.>'

   # Projects this one uses, so affected checks reach their dependents.
   dependsOn: []

   tasks: {}
   ```

5. Add its tasks with the `moon-add-task` skill. Use the standard names that apply: `build`, `lint`, `format`, `test`, `codegen`.
6. Write `libs/<name>/AGENTS.md` with the sections Purpose, Structure, Commands and Local rules, as the `authoring-skills` skill describes. Then add the symlink beside it: `ln -s AGENTS.md libs/<name>/CLAUDE.md`.
7. Check the result:
   - `moon project <name>` shows the source `libs/<name>`, the layer and the tasks.
   - `moon check <name>` passes.
   - `moon run root:validate-agent-files` passes.

## Output

A new project in `libs/<name>/` with `moon.yml`, `AGENTS.md` and a `CLAUDE.md` symlink, listed by `moon projects`.
