//! SeaORM migrations for evie's domain schema. The backend never migrates on
//! its own; `moon run domain-migrations:migrate` applies the pending migrations.
use sea_orm_migration::{MigrationTrait, async_trait};

pub use sea_orm_migration::MigratorTrait;

/// Lists every migration of the domain schema, oldest first.
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        // The domain tables come with the features that need them.
        vec![]
    }
}
