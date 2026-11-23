mod ping;

use evie_proto::evie::ping::v1::ping_service_server::PingServiceServer;
use sea_orm::DatabaseConnection;
use thiserror::Error;
use tokio::{net::TcpListener, task::JoinError};
use tokio_util::sync::CancellationToken;
use tonic::transport::{Server, server::TcpIncoming};
use tonic_health::ServingStatus;

use crate::health::{self, HealthSettings};

#[derive(Debug, Error)]
pub enum ServeError {
    #[error("grpc server failed")]
    Transport(#[from] tonic::transport::Error),

    #[error("database health check task failed")]
    HealthCheck(#[from] JoinError),
}

/// Serves gRPC on `listener` until `shutdown` fires, then lets the open
/// requests finish. The health service reports the server as not serving until
/// a database ping succeeds.
pub async fn serve(
    listener: TcpListener,
    db: DatabaseConnection,
    health: HealthSettings,
    shutdown: CancellationToken,
) -> Result<(), ServeError> {
    let (reporter, health_service) = tonic_health::server::health_reporter();
    // The reporter starts as serving; nothing is known about the database yet.
    reporter
        .set_service_status(health::SERVER, ServingStatus::NotServing)
        .await;
    let watcher = tokio::spawn(health::watch_database(
        db,
        reporter,
        health,
        shutdown.clone(),
    ));

    let served = Server::builder()
        .add_service(health_service)
        .add_service(PingServiceServer::new(ping::PingApi))
        .serve_with_incoming_shutdown(
            TcpIncoming::from(listener),
            shutdown.clone().cancelled_owned(),
        )
        .await;

    // Stops the watcher also when the server failed on its own. The server's
    // error comes first: it is the cause, a watcher failure at most a symptom.
    shutdown.cancel();
    let watched = watcher.await;
    served?;
    Ok(watched?)
}
