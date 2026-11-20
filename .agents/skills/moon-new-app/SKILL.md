---
name: moon-new-app
description: Adds a deployable app to apps/ as a moon project and registers it for releases and image builds with the "Adding an app" checklist. Use when creating a new app, service, server or frontend that ships as its own Docker image.
---

# moon-new-app

An app is deployed as one Docker image. Code that only apps use belongs in a library: use the `moon-new-library` skill.

Until moon templates for new projects exist (issue #27), follow these steps by hand.

## Steps

1. Pick a short kebab-case name. It becomes the project ID. Check it's free: `moon query projects --id <name>` returns no projects.
2. Create `apps/<name>/`. The `apps/*` glob in `.moon/workspace.yml` makes it a project; there's nothing to register with moon.
3. Set up the language's files, such as `Cargo.toml` or `package.json`, as the `rust-conventions` or `typescript-conventions` skill says. A Rust crate takes `version.workspace = true` and `edition.workspace = true`, and joins `members` in the root `Cargo.toml`. A TypeScript project joins the bun workspace by itself; run `bun install` at the root.
4. Write `apps/<name>/moon.yml`:

   ```yaml
   $schema: 'https://moonrepo.dev/schemas/v2/project.json'

   layer: 'application'
   language: 'rust' # or 'typescript'
   project:
     title: '<name>'
     description: '<One sentence: what the app does and who uses it.>'

   # Libraries this app uses, so changes to them affect it.
   dependsOn: []

   tasks: {}
   ```

5. With `language: 'rust'` or `'typescript'`, the project inherits its standard tasks from `.moon/tasks/` (see the `moon-add-task` skill). Add only what's missing with the `moon-add-task` skill, such as a TypeScript project's `build`, or `codegen` and `migrate`.
6. Write `apps/<name>/AGENTS.md` with the sections Purpose, Structure, Commands and Local rules, as the `authoring-skills` skill describes. Then add the symlink beside it: `ln -s AGENTS.md apps/<name>/CLAUDE.md`.
7. Work through the [Adding an app](#adding-an-app) checklist.
8. Check the result:
   - `moon project <name>` shows the source `apps/<name>`, the layer and the tasks.
   - `moon check <name>` passes.
   - `moon run root:validate-agent-files` passes.

## Adding an app

Every app is released with the repository version and published as an image. Issue #19 completes this checklist.

- [ ] **Docker image:** one `Dockerfile` that builds the app's image, in `apps/<name>/`.
- [ ] **Local stack:** add the app as a service to the Docker Compose file (once it exists).
- [ ] **Release version:** add the app's manifest to `extra-files` in `release-please-config.json`, so every release writes the version to it: a `package.json` directly; a Rust crate takes `version.workspace = true` from the root `Cargo.toml`, and its `Cargo.lock` entry is a toml extra-file too, so the lockfile changes in the same release commit. Entries in [references/release-version.md](references/release-version.md).
- [ ] **Image publishing:** add the app to the list of images built and published on every release (once it exists, issues #19 and #42).
- [ ] **Repo map:** name the app in the `apps/` row of the root `AGENTS.md`.

## Output

A new project in `apps/<name>/` with `moon.yml`, `AGENTS.md`, a `CLAUDE.md` symlink and a `Dockerfile`, listed by `moon projects`, and every checklist item done or marked as waiting for its issue.
