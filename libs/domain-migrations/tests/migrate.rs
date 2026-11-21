use domain_migrations::{Migrator, MigratorTrait};
use sea_orm_migration::sea_orm::{Database, DatabaseConnection};
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{ContainerAsync, ImageExt, runners::AsyncRunner},
};

/// The PostgreSQL image the tests run against. Keep it equal to the one in
/// `apps/backend/tests/common/mod.rs`.
const POSTGRES_TAG: &str = "19-alpine";

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

// The two tests below cover every migration in `Migrator`; while it is empty,
// they check only the wiring.
#[tokio::test]
async fn up_applies_every_migration_and_is_idempotent() {
    let (_postgres, db) = start_database().await;

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

#[tokio::test]
async fn down_reverts_every_migration() {
    let (_postgres, db) = start_database().await;
    Migrator::up(&db, None).await.unwrap();

    Migrator::down(&db, None).await.unwrap();

    assert!(
        Migrator::get_applied_migrations(&db)
            .await
            .unwrap()
            .is_empty()
    );
}
