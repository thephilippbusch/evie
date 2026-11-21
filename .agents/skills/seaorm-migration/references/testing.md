# Testing a migration

`libs/domain-migrations/tests/migrate.rs` runs the migrations against PostgreSQL in a container. `up_applies_every_migration_and_is_idempotent` already covers every migration in `Migrator`. Each new migration adds two things:

- a test that `up` creates what the migration adds, through `SchemaManager`'s `has_table`, `has_column` and `has_index`
- a test that `down` removes it again, so `down` keeps working

The first migration also adds the shared `down` test and the `start_database` helper below, and drops the comment above the existing test, which only holds while `Migrator` is empty.

## Shared helper

Move the container start out of the existing test, so every test starts from a fresh database:

```rust
use domain_migrations::{Migrator, MigratorTrait};
use sea_orm_migration::{
    SchemaManager,
    sea_orm::{Database, DatabaseConnection},
};
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{ContainerAsync, ImageExt, runners::AsyncRunner},
};

/// Starts an empty PostgreSQL in a container. Keep the container alive while
/// you use the connection; dropping it removes the container.
async fn start_database() -> (ContainerAsync<Postgres>, DatabaseConnection) {
    let container = Postgres::default()
        .with_tag(POSTGRES_TAG)
        .start()
        .await
        .unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();
    let db = Database::connect(format!(
        "postgres://postgres:postgres@127.0.0.1:{port}/postgres"
    ))
    .await
    .unwrap();
    (container, db)
}
```

## Tests

```rust
#[tokio::test]
async fn down_reverts_every_migration() {
    let (_postgres, db) = start_database().await;
    Migrator::up(&db, None).await.unwrap();

    Migrator::down(&db, None).await.unwrap();

    assert!(Migrator::get_applied_migrations(&db).await.unwrap().is_empty());
}

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
