---
name: moon-new-library
description: Adds a library to libs/ or a development tool to tools/ as a moon project with its moon.yml, tasks and AGENTS.md. Use when creating a new library, crate, package, shared code or development tool, such as a code generator, that is never deployed on its own.
---

# moon-new-library

A library is code that apps use, such as protobuf definitions or a shared crate or package. A tool is a development tool for apps and libraries, such as a code generator, that never runs in production. Neither is deployed. For something that ships as its own Docker image, use the `moon-new-app` skill.

The steps are the same for both. A library goes in `libs/<name>/` with `layer: 'library'`, a tool in `tools/<name>/` with `layer: 'tool'`. moon lets a library depend only on the `library`, `scaffolding` and `configuration` layers, so a code generator that a library lists in `dependsOn` takes `layer: 'scaffolding'`, like `tools/proto-codegen`.

## Steps

1. Pick a short kebab-case name. It becomes the project ID. Check it's free: `moon query projects --id <name>` returns no projects.
2. Generate the project from the `library` template, first with `--dry-run` to see the files, then without:

   ```sh
   moon generate library --to libs/<name> --defaults -- \
     --name <name> --language rust --layer library \
     --description "<One sentence: what the library provides and who uses it.>"
   ```

   For a tool, pass `--to tools/<name> --layer tool`, or `--layer scaffolding`. For TypeScript, pass `--language typescript --package_version <version>` with the current version from `.release-please-manifest.json`; the package is named `@evie/<name>`. The moon MCP `generate` tool takes the same values as `variables`. The template writes `moon.yml`, `AGENTS.md`, the manifest and the entry point. The `libs/*` and `tools/*` globs in `.moon/workspace.yml` make the folder a project.

3. Add the symlink for Claude Code: `ln -s AGENTS.md <folder>/CLAUDE.md`.
4. Join the language's workspace and register the version, as `.agents/skills/moon-new-app/references/release-version.md` shows:
   - Rust: add the folder to `members` in the root `Cargo.toml`, then run `cargo update --workspace` so `Cargo.lock` lists the crate. Every task runs with `--locked`. Add the crate's `Cargo.lock` entry to `extra-files` in `release-please-config.json`.
   - TypeScript: run `bun install` at the root. The root `package.json` workspaces cover `libs/*` only, so a TypeScript tool in `tools/` needs its folder added there first. Add the `package.json` entry to `extra-files`.
5. The project inherits its standard tasks from `.moon/tasks/`. Add only what's missing with the `moon-add-task` skill, such as `codegen` or `migrate`. List each new task under Commands in `AGENTS.md`, and the projects it uses under `dependsOn` in `moon.yml`.
6. Write the real code with the `rust-conventions` or `typescript-conventions` skill. A TypeScript library has no barrel file: replace the placeholder `src/<name>.ts` with modules imported as `@evie/<name>/<module>`. Keep `AGENTS.md` current as it grows, as the `authoring-skills` skill describes.
7. Check the result:
   - `moon project <name>` shows the source `libs/<name>` or `tools/<name>`, the layer and the tasks.
   - `moon check <name>` passes.
   - `moon run root:validate-agent-files` passes.

## Output

A new project in `libs/<name>/` or `tools/<name>/`, generated from the `library` template, with a `CLAUDE.md` symlink, listed by `moon projects`.
