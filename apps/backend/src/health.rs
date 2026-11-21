use std::time::Duration;

use sea_orm::DatabaseConnection;
use tokio::time::MissedTickBehavior;
use tokio_util::sync::CancellationToken;
use tonic_health::{ServingStatus, server::HealthReporter};

/// The status of the whole server, by the gRPC health checking protocol.
pub(crate) const SERVER: &str = "";

/// How often and how patiently the server checks its database.
#[derive(Debug, Clone, Copy)]
pub struct HealthSettings {
    /// Time between two database pings.
    pub interval: Duration,
    /// How long one ping may take before it counts as failed.
    pub timeout: Duration,
}

impl Default for HealthSettings {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(5),
            timeout: Duration::from_secs(2),
        }
    }
}

/// Pings the database every `settings.interval` and reports the server as
/// serving while the pings succeed, and as not serving while they fail. Logs
/// each change once. Returns when `shutdown` fires.
pub(crate) async fn watch_database(
    db: DatabaseConnection,
    reporter: HealthReporter,
    settings: HealthSettings,
    shutdown: CancellationToken,
) {
    let mut ticks = tokio::time::interval(settings.interval);
    ticks.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let mut serving = None;

    loop {
        tokio::select! {
            () = shutdown.cancelled() => return,
            _ = ticks.tick() => {}
        }

        // A ping only reads, so dropping it on shutdown is safe.
        let ping = tokio::select! {
            () = shutdown.cancelled() => return,
            ping = tokio::time::timeout(settings.timeout, db.ping()) => ping,
        };
        let reachable = matches!(ping, Ok(Ok(())));
        if serving == Some(reachable) {
            continue;
        }

        match ping {
            Ok(Ok(())) => tracing::info!("database reachable, serving"),
            Ok(Err(err)) => tracing::warn!(error = %err, "database unreachable, not serving"),
            Err(_) => {
                tracing::warn!(timeout = ?settings.timeout, "database ping timed out, not serving")
            }
        }
        let status = if reachable {
            ServingStatus::Serving
        } else {
            ServingStatus::NotServing
        };
        reporter.set_service_status(SERVER, status).await;
        serving = Some(reachable);
    }
}
