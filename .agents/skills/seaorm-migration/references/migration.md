# Writing the migration

A migration is one file in `libs/domain-migrations/src/`, named for its module, with one `Migration` struct. `sea_orm_migration::schema` has a helper per column type, such as `pk_uuid`, `string`, `integer`, `text_null` and `timestamp_with_time_zone`. Helpers without `_null` make the column `NOT NULL`.

## Create a table

```rust
// libs/domain-migrations/src/m20261121_143000_create_place.rs
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Place::Table)
                    .col(pk_uuid(Place::Id))
                    .col(string(Place::Name))
                    .col(integer(Place::Capacity))
                    // The BetterAuth user ID: opaque, with no foreign key into the auth schema.
                    .col(text(Place::HostUserId))
                    .col(timestamp_with_time_zone_default_now(Place::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_place_host_user_id")
                    .table(Place::Table)
                    .col(Place::HostUserId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Dropping the table drops its indexes too.
        manager
            .drop_table(Table::drop().table(Place::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Place {
    Table,
    Id,
    Name,
    Capacity,
    HostUserId,
    CreatedAt,
}
```

`DeriveIden` turns `Place::Table` into `place` and `Place::HostUserId` into `host_user_id`.

## Register it

```rust
// libs/domain-migrations/src/lib.rs
mod m20261121_143000_create_place;

// in `impl MigratorTrait for Migrator`
fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
        // …the migrations before it
        Box::new(m20261121_143000_create_place::Migration),
    ]
}
```

The list decides the order; the newest migration is always last. The first migration also replaces the comment in the empty `vec![]`.

## Add a column

A later migration declares only the identifiers it needs. A new `NOT NULL` column on a table with rows needs a default.

```rust
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
        .alter_table(
            Table::alter()
                .table(Place::Table)
                .add_column(boolean(Place::Accessible).default(false))
                .to_owned(),
        )
        .await
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
        .alter_table(
            Table::alter()
                .table(Place::Table)
                .drop_column(Place::Accessible)
                .to_owned(),
        )
        .await
}

#[derive(DeriveIden)]
enum Place {
    Table,
    Accessible,
}
```

## Reference another domain table

```rust
.col(uuid(Booking::PlaceId))
.foreign_key(
    ForeignKey::create()
        .name("fk_booking_place_id")
        .from(Booking::Table, Booking::PlaceId)
        .to(Place::Table, Place::Id)
        .on_delete(ForeignKeyAction::Cascade),
)
```

Never point a foreign key at a table in the `auth` schema.

## Add an enum

A PostgreSQL enum is a type of its own: create it before the table that uses it, and drop it after.

```rust
use sea_orm_migration::{prelude::*, schema::*, sea_query::extension::postgres::Type};

// in `up`, before the table
manager
    .create_type(
        Type::create()
            .as_enum(BookingStatus::Enum)
            .values([BookingStatus::Pending, BookingStatus::Confirmed])
            .to_owned(),
    )
    .await?;

// the column, in `Table::create()`
.col(enumeration(
    Booking::Status,
    BookingStatus::Enum,
    [BookingStatus::Pending, BookingStatus::Confirmed],
))

// in `down`, after dropping the table
manager
    .drop_type(Type::drop().name(BookingStatus::Enum).to_owned())
    .await

#[derive(DeriveIden)]
enum BookingStatus {
    #[sea_orm(iden = "booking_status")]
    Enum,
    Pending,
    Confirmed,
}
```

## Things `down` must undo

| `up`                         | `down`                                     |
| ---------------------------- | ------------------------------------------ |
| `create_table`               | `drop_table`                               |
| `alter_table` + `add_column` | `alter_table` + `drop_column`              |
| `create_index`               | `drop_index` (unless the table is dropped) |
| `create_type` (enum)         | `drop_type`, after the tables that use it  |
| `drop_column`                | `add_column` with the old type, empty      |

Run several statements in `down` in the reverse order of `up`.
