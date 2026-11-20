# migration

## Purpose

SeaORM migrations for evie's domain schema, owned by SeaORM. The `auth` schema belongs to Drizzle in the frontend; no migration here touches it.
The backend never migrates on start: it must run and report its health while PostgreSQL is down. The `migrate` task applies the pending migrations.

## Structure

| Path               | Contents                                                    |
| ------------------ | ----------------------------------------------------------- |
| `src/lib.rs`       | `Migrator`, which lists every migration, oldest first       |
| `src/main.rs`      | The SeaORM migration CLI, run by the `migrate` task         |
| `tests/migrate.rs` | Applies every migration to a fresh PostgreSQL container     |
| `Cargo.toml`       | Crate manifest; version and edition come from the workspace |

## Commands

The tasks come from [`.moon/tasks/rust.yml`](../../.moon/tasks/rust.yml), except `migrate`.

| Command                      | What it does                                                                        |
| ---------------------------- | ----------------------------------------------------------------------------------- |
| `moon run migration:migrate` | Apply the pending migrations to the database in `DATABASE_URL`                      |
| `moon run migration:build`   | Compile the crate                                                                   |
| `moon run migration:lint`    | Lint with clippy                                                                    |
| `moon run migration:check`   | Run `lint` (clippy also checks the types), then check the formatting with rustfmt   |
| `moon run migration:test`    | Run the tests; they start PostgreSQL with testcontainers, so Docker must be running |
| `moon run migration:format`  | Format the crate with rustfmt                                                       |

For other CLI commands, run it from `libs/migration/`, e.g. `cargo run -- status` or `cargo run -- down`.

## Local rules

- Follow the `rust-conventions` skill.
- Never edit a migration that has been merged to `main`; add a new one that changes the schema further.
- Add each migration to `Migrator::migrations` in order, and keep `down` working.
- The migration CLI reads a `.env` file in this folder if one exists. Never read or write it; pass `DATABASE_URL` in the environment.
