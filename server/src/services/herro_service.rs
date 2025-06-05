use super::super::types::State;
use herro::{HerroRequest, HerroResponse, herro_server::Herro};
use tonic::{Request, Response, Status};

pub mod herro {
    tonic::include_proto!("herro");
}

#[derive(Debug, Default)]
pub struct HerroService {
    pub state: State,
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
