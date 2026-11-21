---
name: moon-new-app
description: Adds a deployable app to apps/ as a moon project and registers it for releases and image builds with the "Adding an app" checklist. Use when creating a new app, service, server or frontend that ships as its own Docker image.
---

# moon-new-app

An app is deployed as one Docker image. Code that only apps use belongs in a library: use the `moon-new-library` skill.

## Steps

1. Pick a short kebab-case name. It becomes the project ID. Check it's free: `moon query projects --id <name>` returns no projects.
2. Generate the project from the `app` template, first with `--dry-run` to see the files, then without:

   ```sh
   moon generate app --to apps/<name> --defaults -- \
     --name <name> --language rust \
     --description "<One sentence: what the app does and who uses it.>"
   ```

   For TypeScript, pass `--language typescript --package_version <version>` with the current version from `.release-please-manifest.json`. The moon MCP `generate` tool takes the same values as `variables`. The template writes `moon.yml`, `AGENTS.md`, the manifest and the entry point. The `apps/*` glob in `.moon/workspace.yml` makes the folder a project.

3. Add the symlink for Claude Code: `ln -s AGENTS.md apps/<name>/CLAUDE.md`.
4. Join the language's workspace:
   - Rust: add `apps/<name>` to `members` in the root `Cargo.toml`, then run `cargo update --workspace` so `Cargo.lock` lists the crate. Every task runs with `--locked`.
   - TypeScript: run `bun install` at the root.
5. The project inherits its standard tasks from `.moon/tasks/`. Add only what's missing with the `moon-add-task` skill, such as a TypeScript app's `build`. List each new task under Commands in `AGENTS.md`.
6. Write the app's real code with the `rust-conventions` or `typescript-conventions` skill. Keep `AGENTS.md` current as it grows: Structure, Commands and Local rules, as the `authoring-skills` skill describes.
7. Work through the [Adding an app](#adding-an-app) checklist.
8. Check the result:
   - `moon project <name>` shows the source `apps/<name>`, the layer `application` and the tasks.
   - `moon check <name>` passes.
   - `moon run root:validate-agent-files` passes.

## Adding an app

Every app is released with the repository version and published as an image. Issue #19 completes this checklist.

- [ ] **Docker image:** one multi-stage `Dockerfile` in `apps/<name>/` that builds the app's image, with the repository root as the build context. Start from the example for the language: `apps/backend/Dockerfile` for Rust, `apps/frontend/Dockerfile` for TypeScript. Use the versions pinned in `.prototools`, and run the app as a non-root user. Check it with `docker build -f apps/<name>/Dockerfile .`. A Rust app is a new Cargo workspace member, so also add its folder to the `COPY` lines of every other Rust `Dockerfile`; cargo resolves the whole workspace.
- [ ] **Local stack:** add the app as a service to the root `compose.yaml`, shaped like the `backend` and `frontend` services: `build` with `context: .` and the `dockerfile`, the `environment` the app needs, and `depends_on` for the services it calls. Publish a host port only if a person opens the app, and bind it to `127.0.0.1`. Check that `docker compose up --build` starts it.
- [ ] **Release version:** add the app's manifest to `extra-files` in `release-please-config.json`, so every release writes the version to it: a `package.json` directly; a Rust crate takes `version.workspace = true` from the root `Cargo.toml`, and its `Cargo.lock` entry is a toml extra-file too, so the lockfile changes in the same release commit. Entries in [references/release-version.md](references/release-version.md).
- [ ] **Image publishing:** add the app to the list of images built and published on every release (once it exists, issues #19 and #42).
- [ ] **Repo map:** name the app in the `apps/` row of the root `AGENTS.md`.

## Output

A new project in `apps/<name>/`, generated from the `app` template, with a `CLAUDE.md` symlink, listed by `moon projects`, and every checklist item done or marked as waiting for its issue.
