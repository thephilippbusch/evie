mod common;

use common::TestServer;
use tonic_health::pb::health_check_response::ServingStatus;

#[tokio::test]
async fn health_reports_serving_when_the_database_is_reachable() {
    let (_postgres, database_url) = common::start_postgres().await;
    let mut server = TestServer::start(&database_url).await;

    server.wait_for(ServingStatus::Serving).await;

    assert_eq!(server.status().await, ServingStatus::Serving);
    server.stop().await;
}

#[tokio::test]
async fn health_reports_not_serving_when_the_database_is_unreachable() {
    let database_url = common::unreachable_database_url().await;
    let mut server = TestServer::start(&database_url).await;

    // Long enough for several pings to time out.
    tokio::time::sleep(common::HEALTH.timeout * 4).await;

    assert_eq!(server.status().await, ServingStatus::NotServing);
    server.stop().await;
}
