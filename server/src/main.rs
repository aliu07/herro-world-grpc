use herro::{
    HerroRequest, HerroResponse,
    herro_server::{Herro, HerroServer},
};
use tonic::{Request, Response, Status, transport::Server};

pub mod herro {
    tonic::include_proto!("herro");
}

#[derive(Debug, Default)]
pub struct HerroService {}

#[tonic::async_trait]
impl Herro for HerroService {
    async fn say_herro(
        &self,
        request: Request<HerroRequest>,
    ) -> Result<Response<HerroResponse>, Status> {
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
    let herro_service = HerroService::default();

    Server::builder()
        .add_service(HerroServer::new(herro_service))
        .serve(address)
        .await?;

    Ok(())
}
