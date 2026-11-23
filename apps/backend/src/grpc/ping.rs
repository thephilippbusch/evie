use evie_proto::evie::ping::v1::{PingRequest, PingResponse, ping_service_server};
use tonic::{Request, Response, Status};

/// Serves `evie.ping.v1.PingService`.
pub(crate) struct PingApi;

#[tonic::async_trait]
impl ping_service_server::PingService for PingApi {
    async fn ping(&self, _request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        Ok(Response::new(PingResponse {
            version: env!("CARGO_PKG_VERSION").to_owned(),
        }))
    }
}
