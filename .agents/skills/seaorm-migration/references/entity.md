# The backend entity

The backend reads and writes a table through a SeaORM entity: a hand-written `Model` that matches the migration column for column. The entity is private to the feature module that owns the table; other modules go through that feature's service.

## First table the backend uses

Once, the first time the backend touches a domain table:

1. Turn on the column types the backend needs, e.g. in `apps/backend/`: `cargo add sea-orm --features with-uuid,with-chrono` and `cargo add uuid --features v4`.
2. Add the migrations as a dev dependency, so integration tests can migrate their database: `cargo add --dev --path ../../libs/domain-migrations`. The backend itself never migrates.
3. Check that `'domain-migrations'` is in `dependsOn` in `apps/backend/moon.yml`, and add it if it's missing. It makes a migration change rerun the backend's checks, and `moon docker scaffold` needs it for the `migrate` image.
4. Update the sentence about `domain-migrations` under Purpose in `apps/backend/AGENTS.md`, so it names the dev dependency.
5. Commit `Cargo.lock` with the change.

## Entity

`apps/backend/src/<feature>/entity.rs`, declared as a private `mod entity;` in `src/<feature>.rs`, the way the `rust-conventions` skill lays out modules. The parent module holds the feature's error and re-exports what the rest of the crate uses; add it to `src/lib.rs` as a private `mod place;`:

```rust
// apps/backend/src/place.rs
//! Places to sleep that hosts offer at an event.
mod entity;
mod service;

use thiserror::Error;
use uuid::Uuid;

pub(crate) use service::{Place, PlaceService};

#[derive(Debug, Error)]
pub(crate) enum PlaceError {
    #[error("place {0} not found")]
    NotFound(Uuid),

    #[error("database error")]
    Database(#[from] sea_orm::DbErr),
}
```

```rust
// apps/backend/src/place/entity.rs
use sea_orm::entity::prelude::*;

/// A row of the `place` table, created by `m20261121_143000_create_place`.
/// `pub` as `DeriveEntityModel` expects; the private `entity` module keeps it
/// inside the feature.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "place")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub capacity: i32,
    /// The BetterAuth user ID, opaque to the domain.
    pub host_user_id: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
```

| Column helper in the migration | Rust type in the entity |
| ------------------------------ | ----------------------- |
| `pk_uuid`, `uuid`              | `Uuid`                  |
| `string`, `text`               | `String`                |
| `integer`                      | `i32`                   |
| `big_integer`                  | `i64`                   |
| `boolean`                      | `bool`                  |
| `timestamp_with_time_zone…`    | `DateTimeWithTimeZone`  |
| any `…_null` helper            | `Option<…>`             |

## Using it

The feature's service holds the `DatabaseConnection` and returns domain types, not the `Model`:

```rust
// apps/backend/src/place/service.rs
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

use super::{PlaceError, entity};

pub(crate) struct Place {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) capacity: i32,
}

#[derive(Clone)]
pub(crate) struct PlaceService {
    db: DatabaseConnection,
}

impl PlaceService {
    pub(crate) fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub(crate) async fn get(&self, id: Uuid) -> Result<Place, PlaceError> {
        let row = entity::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(PlaceError::NotFound(id))?;
        Ok(Place {
            id: row.id,
            name: row.name,
            capacity: row.capacity,
        })
    }
}
```

`PlaceError` in `src/place.rs` wraps `sea_orm::DbErr` with `#[from]`, as the `rust-conventions` skill shows for errors.

## Integration tests against the schema

Tests that read or write domain tables migrate their container first. Add a helper next to `start_postgres` in `apps/backend/tests/common/mod.rs`:

```rust
use domain_migrations::{Migrator, MigratorTrait};

/// Starts PostgreSQL in a container and applies every domain migration.
pub async fn start_migrated_postgres() -> (ContainerAsync<Postgres>, String) {
    let (container, url) = start_postgres().await;
    let db = backend::connect(&url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();
    (container, url)
}
```
