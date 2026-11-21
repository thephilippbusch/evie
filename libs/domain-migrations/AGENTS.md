# domain-migrations

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

`migrate` is defined in [`moon.yml`](moon.yml); the other tasks come from [`.moon/tasks/rust.yml`](../../.moon/tasks/rust.yml), shared by every Rust crate.

| Command                              | What it does                                                                        |
| ------------------------------------ | ----------------------------------------------------------------------------------- |
| `moon run domain-migrations:migrate` | Apply the pending migrations to the database in `DATABASE_URL`                      |
| `moon run domain-migrations:build`   | Compile the crate                                                                   |
| `moon run domain-migrations:lint`    | Lint with clippy                                                                    |
| `moon run domain-migrations:check`   | Run `lint` (clippy also checks the types), then check the formatting with rustfmt   |
| `moon run domain-migrations:test`    | Run the tests; they start PostgreSQL with testcontainers, so Docker must be running |
| `moon run domain-migrations:format`  | Format the crate with rustfmt                                                       |

For other CLI commands, run it from `libs/domain-migrations/`, e.g. `cargo run -- status` or `cargo run -- down`.

## Local rules

- Follow the `rust-conventions` skill.
- Never edit a migration that has been merged to `main`; add a new one that changes the schema further.
- Add each migration to `Migrator::migrations` in order, and keep `down` working.
- The migration CLI reads a `.env` file in this folder if one exists. Never read or write it; pass `DATABASE_URL` in the environment.
