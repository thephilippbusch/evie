use domain_migrations::{Migrator, MigratorTrait};
use sea_orm_migration::sea_orm::Database;
use testcontainers_modules::{
    postgres::Postgres, testcontainers::ImageExt, testcontainers::runners::AsyncRunner,
};

/// The PostgreSQL image the tests run against. Keep it equal to the one in
/// `apps/backend/tests/common/mod.rs`.
const POSTGRES_TAG: &str = "19-alpine";

// Until the first migration exists, this checks only the wiring: `up` creates
// the migration table on a fresh database and runs again without error.
#[tokio::test]
async fn up_applies_every_migration_and_is_idempotent() {
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

    Migrator::up(&db, None).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    assert!(
        Migrator::get_pending_migrations(&db)
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        Migrator::get_applied_migrations(&db).await.unwrap().len(),
        Migrator::migrations().len()
    );
}
