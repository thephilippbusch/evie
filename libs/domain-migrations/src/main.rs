//! The SeaORM migration CLI, e.g. `cargo run -- up`. It reads the database
//! from `DATABASE_URL`.
use sea_orm_migration::cli;

#[tokio::main]
async fn main() {
    cli::run_cli(domain_migrations::Migrator).await;
}
