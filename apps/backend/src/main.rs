//! Startup and wiring only; see `lib.rs`.
use anyhow::Context;
use backend::{Config, HealthSettings};
use tokio::{
    net::TcpListener,
    signal::unix::{SignalKind, signal},
};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env().context("reading the config")?;
    backend::init_tracing(config.log_format).context("reading RUST_LOG")?;

    let db = backend::connect(&config.database_url)
        .await
        .context("setting up the database connection")?;
    let listener = TcpListener::bind(config.grpc_addr)
        .await
        .with_context(|| format!("binding {}", config.grpc_addr))?;
    let mut interrupt = signal(SignalKind::interrupt()).context("listening for SIGINT")?;
    let mut terminate = signal(SignalKind::terminate()).context("listening for SIGTERM")?;

    let shutdown = CancellationToken::new();
    let signals = tokio::spawn({
        let shutdown = shutdown.clone();
        async move {
            tokio::select! {
                _ = interrupt.recv() => {}
                _ = terminate.recv() => {}
            }
            tracing::info!("shutting down");
            shutdown.cancel();
        }
    });

    tracing::info!(addr = %config.grpc_addr, "serving gRPC");
    let served = backend::serve(listener, db, HealthSettings::default(), shutdown).await;
    signals.abort();
    served.context("serving gRPC")
}
