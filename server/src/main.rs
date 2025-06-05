use herro::admin_server::{Admin, AdminServer};
use herro::{
    HerroRequest, HerroResponse,
    herro_server::{Herro, HerroServer},
};
use tonic::{Request, Response, Status, transport::Server};

pub mod herro {
    tonic::include_proto!("herro");
}

type State = std::sync::Arc<tokio::sync::RwLock<u64>>;

#[derive(Debug, Default)]
pub struct AdminService {
    state: State,
}

#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_request_count(
        &self,
        _request: tonic::Request<herro::GetCountRequest>,
    ) -> Result<tonic::Response<herro::CounterResponse>, tonic::Status> {
        let count = self.state.read().await;
        let response = herro::CounterResponse { count: *count };
        Ok(tonic::Response::new(response))
    }
}

#[derive(Debug, Default)]
pub struct HerroService {
    state: State,
}

impl HerroService {
    async fn increment_counter(&self) {
        let mut count = self.state.write().await;
        *count += 1;
        println!("Request count: {}", *count);
    }
}

#[tonic::async_trait]
impl Herro for HerroService {
    async fn say_herro(
        &self,
        request: Request<HerroRequest>,
    ) -> Result<Response<HerroResponse>, Status> {
        // Increment counter
        self.increment_counter().await;

        let req = request.into_inner();

        // We hate Alec
        if req.name == "Alec" {
            Err(Status::new(
                tonic::Code::InvalidArgument,
                "ALL MY HOMIES HATE ALEC, terminating your session",
            ))
        } else {
            Ok(Response::new(herro::HerroResponse {
                greeting: format!["Hello, {}!", req.name],
            }))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse().unwrap();

    let state = State::default();

    let herro_service = HerroService {
        state: state.clone(),
    };

    let admin_service = AdminService {
        state: state.clone(),
    };

    Server::builder()
        .add_service(HerroServer::new(herro_service))
        .add_service(AdminServer::new(admin_service))
        .serve(address)
        .await?;

    Ok(())
}
