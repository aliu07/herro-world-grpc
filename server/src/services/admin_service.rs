use super::super::types::State;
use admin::admin_server::Admin;

pub mod admin {
    tonic::include_proto!("admin");
}

#[derive(Debug, Default)]
pub struct AdminService {
    pub state: State,
}

#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_request_count(
        &self,
        _request: tonic::Request<admin::GetCountRequest>,
    ) -> Result<tonic::Response<admin::CounterResponse>, tonic::Status> {
        let count = self.state.read().await;
        let response = admin::CounterResponse { count: *count };
        Ok(tonic::Response::new(response))
    }
}
