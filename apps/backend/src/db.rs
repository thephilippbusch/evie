use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

/// How long opening a connection or waiting for a free one may take.
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(2);

/// Creates the connection pool for `database_url`. The pool connects on first
/// use, so the backend starts, and reports not serving, while PostgreSQL is
/// down. Fails only on an invalid URL.
pub async fn connect(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut options = ConnectOptions::new(database_url);
    options
        .connect_lazy(true)
        .connect_timeout(CONNECTION_TIMEOUT)
        .acquire_timeout(CONNECTION_TIMEOUT);
    Database::connect(options).await
}
