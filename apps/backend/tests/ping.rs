mod common;

use common::TestServer;
use evie_proto::evie::ping::v1::{PingRequest, ping_service_client::PingServiceClient};

#[tokio::test]
async fn ping_returns_the_backend_version() {
    let database_url = common::unreachable_database_url().await;
    let server = TestServer::start(&database_url).await;
    let mut client = PingServiceClient::new(server.channel());

    let response = client.ping(PingRequest {}).await.unwrap().into_inner();

    assert_eq!(response.version, env!("CARGO_PKG_VERSION"));
    drop(client);
    server.stop().await;
}
