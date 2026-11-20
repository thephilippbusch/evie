use std::time::Duration;

use backend::HealthSettings;
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{ContainerAsync, ImageExt, runners::AsyncRunner},
};
use tokio::{net::TcpListener, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use tonic::transport::Channel;
use tonic_health::pb::{
    HealthCheckRequest, health_check_response::ServingStatus, health_client::HealthClient,
};

/// The PostgreSQL image the tests run against.
const POSTGRES_TAG: &str = "19-alpine";

/// Fast checks, so the tests see a status change quickly. A failed ping takes
/// the whole timeout, since the pool retries the connection until then.
pub const HEALTH: HealthSettings = HealthSettings {
    interval: Duration::from_millis(100),
    timeout: Duration::from_millis(300),
};

/// How long a test waits for an expected status.
const STATUS_TIMEOUT: Duration = Duration::from_secs(10);

/// Starts PostgreSQL in a container. Keep the container alive while you use
/// the URL; dropping it removes the container.
pub async fn start_postgres() -> (ContainerAsync<Postgres>, String) {
    let container = Postgres::default()
        .with_tag(POSTGRES_TAG)
        .start()
        .await
        .unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();
    let url = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");
    (container, url)
}

/// A URL to a local port that nothing listens on.
pub async fn unreachable_database_url() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres")
}

/// The backend, served on a free local port.
pub struct TestServer {
    client: HealthClient<Channel>,
    shutdown: CancellationToken,
    served: JoinHandle<Result<(), backend::ServeError>>,
}

impl TestServer {
    pub async fn start(database_url: &str) -> Self {
        let db = backend::connect(database_url).await.unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let shutdown = CancellationToken::new();
        let served = tokio::spawn(backend::serve(listener, db, HEALTH, shutdown.clone()));
        let channel = Channel::from_shared(format!("http://{addr}"))
            .unwrap()
            .connect()
            .await
            .unwrap();
        let client = HealthClient::new(channel);
        Self {
            client,
            shutdown,
            served,
        }
    }

    /// The server's status, from the health service's `Check` RPC.
    pub async fn status(&mut self) -> ServingStatus {
        let response = self
            .client
            .check(HealthCheckRequest {
                service: String::new(),
            })
            .await
            .unwrap();
        response.into_inner().status()
    }

    /// Waits until the health service's `Watch` stream reports `expected`.
    /// Panics after `STATUS_TIMEOUT`.
    pub async fn wait_for(&mut self, expected: ServingStatus) {
        let mut updates = self
            .client
            .watch(HealthCheckRequest {
                service: String::new(),
            })
            .await
            .unwrap()
            .into_inner();
        let reached = async {
            while let Some(update) = updates.message().await.unwrap() {
                if update.status() == expected {
                    return;
                }
            }
            panic!("the watch stream ended before reporting {expected:?}");
        };
        tokio::time::timeout(STATUS_TIMEOUT, reached)
            .await
            .unwrap_or_else(|_| panic!("the server didn't report {expected:?} in time"));
    }

    /// Stops the server and checks that it shut down without an error.
    pub async fn stop(self) {
        self.shutdown.cancel();
        self.served.await.unwrap().unwrap();
    }
}
