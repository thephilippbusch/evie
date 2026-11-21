# The backend entity

The backend reads and writes a table through a SeaORM entity. Entities will be generated from the migrations by a `codegen` task, never written by hand; #127 adds that task and fills in this reference. Until #127 lands, a change that needs a backend entity waits for it.

What holds however the entity is made:

## First table the backend uses

Once, the first time the backend touches a domain table:

1. Turn on the column types the backend needs, e.g. in `apps/backend/`: `cargo add sea-orm --features with-uuid,with-chrono` and `cargo add uuid --features v4`.
2. Add the migrations as a dev dependency, so integration tests can migrate their database: `cargo add --dev --path ../../libs/domain-migrations`. The backend itself never migrates.
3. Check that `'domain-migrations'` is in `dependsOn` in `apps/backend/moon.yml`, and add it if it's missing. It makes a migration change rerun the backend's checks, and `moon docker scaffold` needs it for the `migrate` image.
4. Update the sentence about `domain-migrations` under Purpose in `apps/backend/AGENTS.md`, so it names the dev dependency.
5. Commit `Cargo.lock` with the change.

## Using it

- Only the feature module that owns the table uses its entity; other modules go through that feature's service.
- The feature's service holds the `DatabaseConnection` and returns domain types, never the entity's `Model`.
- The feature's `thiserror` enum wraps `sea_orm::DbErr` with `#[from]`, as the `rust-conventions` skill shows for errors.
- Integration tests that read or write domain tables apply `Migrator::up` to their container before the test. The first such test adds `start_migrated_postgres()` next to `start_postgres` in `apps/backend/tests/common/mod.rs`: it starts PostgreSQL, applies every migration and returns the container and URL; every later test reuses it.
