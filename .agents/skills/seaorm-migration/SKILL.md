---
name: seaorm-migration
description: Adds a SeaORM migration to evie's domain schema in libs/domain-migrations, with a working down and its tests, and prepares the backend to read the table. Use when adding or changing a domain table, column, index or enum, or when asked to write, apply or roll back a database migration.
---

# SeaORM migration

The `domain-migrations` crate owns the domain schema; the backend never migrates on start. Read `libs/domain-migrations/AGENTS.md` first.
Not for the `auth` schema: Drizzle in the frontend owns it.

| Read                                               | When you                                                     |
| -------------------------------------------------- | ------------------------------------------------------------ |
| [references/migration.md](references/migration.md) | write the migration file: create, alter, index, enum, `down` |
| [references/testing.md](references/testing.md)     | extend `tests/migrate.rs`                                    |
| [references/entity.md](references/entity.md)       | read or write the table from the backend (entities: #127)    |

## Steps

1. Name the migration `m<YYYYMMDD>_<HHMMSS>_<change>`, with the current UTC time from `date -u +%Y%m%d_%H%M%S`. `<change>` is snake_case and starts with a verb: `create_place`, `add_place_capacity`.
2. Create `libs/domain-migrations/src/<name>.rs` with one `Migration`: `up` makes the change, `down` undoes it. Write it by hand from [references/migration.md](references/migration.md); the CLI's `generate` template doesn't match this crate.
3. Register it in `libs/domain-migrations/src/lib.rs`: a `mod <name>;` line, and `Box::new(<name>::Migration)` as the last entry of `Migrator::migrations`.
4. Extend `libs/domain-migrations/tests/migrate.rs` with [references/testing.md](references/testing.md).
5. Check with `moon run domain-migrations:check domain-migrations:test`. The tests start PostgreSQL with testcontainers, so Docker must run. Use the `moon-run` skill.
6. Apply it to your local database: `moon run domain-migrations:migrate`, with `DATABASE_URL` set in the environment.
7. Check `down` against the same database: in `libs/domain-migrations/`, run `cargo run -- down`, then `moon run domain-migrations:migrate` again.
8. If the backend reads or writes the table, follow [references/entity.md](references/entity.md).

## Schema conventions

- Table names are singular snake_case (`place`, `booking`); column names snake_case.
- Name tables and columns through a `DeriveIden` enum, not string literals, so a typo fails to compile.
- Primary keys are `id uuid`, set by the backend.
- Points in time are `timestamptz` (`timestamp_with_time_zone`).
- A BetterAuth user ID is an opaque `text` column, such as `host_user_id`, with no foreign key: the `auth` tables belong to Drizzle.
- Foreign keys between domain tables are fine; name them `fk_<table>_<column>`. Indexes are `idx_<table>_<column>`.
- One schema change per migration. On PostgreSQL, SeaORM runs each migration in a transaction, so a failing `up` leaves nothing behind.

## Rules

- Never edit, rename or delete a migration that is merged to `main`; add a new one that changes the schema further.
- Never touch the `auth` schema or its tables.
- `down` restores the schema as it was before `up`. When `up` drops data, `down` recreates the structure empty.
- Never read or write `.env` files; pass `DATABASE_URL` in the environment.
- Follow the `rust-conventions` skill.

## Output

The new migration file, its line in `src/lib.rs`, the extended `tests/migrate.rs` and, if the backend uses the table, the steps of [references/entity.md](references/entity.md), with `moon run domain-migrations:check domain-migrations:test` passing.
