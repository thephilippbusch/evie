# Testing a migration

`libs/domain-migrations/tests/migrate.rs` runs the migrations against PostgreSQL in a container. Its `start_database()` helper starts a fresh database for each test, and two tests already cover every migration in `Migrator`: `up_applies_every_migration_and_is_idempotent` and `down_reverts_every_migration`. Each new migration adds two tests of its own, both starting from `start_database()`:

- a test that `up` creates what the migration adds, through `SchemaManager`'s `has_table`, `has_column` and `has_index`
- a test that `down` removes it again, so `down` keeps working

## Example

The first migration that uses `SchemaManager` adds it to the imports: `use sea_orm_migration::{SchemaManager, sea_orm::{Database, DatabaseConnection}};`.

```rust
#[tokio::test]
async fn create_place_adds_the_place_table() {
    let (_postgres, db) = start_database().await;

    Migrator::up(&db, None).await.unwrap();

    let schema = SchemaManager::new(&db);
    assert!(schema.has_table("place").await.unwrap());
    assert!(schema.has_column("place", "host_user_id").await.unwrap());
    assert!(schema.has_index("place", "idx_place_host_user_id").await.unwrap());
}

#[tokio::test]
async fn create_place_down_drops_the_place_table() {
    let (_postgres, db) = start_database().await;
    Migrator::up(&db, None).await.unwrap();

    Migrator::down(&db, Some(1)).await.unwrap();

    assert!(!SchemaManager::new(&db).has_table("place").await.unwrap());
}
```

`Migrator::down(&db, Some(1))` rolls back only the newest migration. A test of an older migration's `down` rolls back as many steps as it has migrations after it.

Run the tests with `moon run domain-migrations:test`, or one of them with `moon run domain-migrations:test -- create_place`.
