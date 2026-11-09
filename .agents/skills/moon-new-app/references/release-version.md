# Registering an app's version with release-please

Every release writes the repository version into the files listed in `extra-files` of the `"."` package in `release-please-config.json`. Each file needs the version field already: set it to the current version from `.release-please-manifest.json`. An entry whose path or field doesn't exist is skipped with only a warning.

## TypeScript app

Add one entry per app:

```json
{ "type": "json", "path": "apps/<name>/package.json", "jsonpath": "$.version" }
```

## Rust app

The crate takes its version from the workspace: `version.workspace = true` in `apps/<name>/Cargo.toml`, and `version` under `[workspace.package]` in the root `Cargo.toml`.

The first Rust app adds the root `Cargo.toml` entry once:

```json
{
  "type": "toml",
  "path": "Cargo.toml",
  "jsonpath": "$.workspace.package.version"
}
```

Every crate that uses `version.workspace = true`, apps and libraries alike, also adds its `Cargo.lock` entry, with the crate's `name` from its `Cargo.toml`:

```json
{
  "type": "toml",
  "path": "Cargo.lock",
  "jsonpath": "$.package[?(@.name.value=='<crate>')].version"
}
```

Without it, the release commit leaves `Cargo.lock` at the old version, and the next `cargo build --locked` fails. Keep the `.value`: release-please's toml parser wraps every value in an object, so `@.name=='<crate>'` matches nothing and only logs a warning.

A workflow step that runs `cargo update --workspace` on the release PR branch would also work, but release-please rewrites that branch on every run and drops the step's commit.

## Check

- `uvx check-jsonschema --schemafile https://raw.githubusercontent.com/googleapis/release-please/main/schemas/config.json release-please-config.json` passes.
- The next release PR changes every registered file to the new version.
