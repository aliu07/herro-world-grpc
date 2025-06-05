use middleware::authentication::check_auth;
use services::admin_service::{AdminService, admin::admin_server::AdminServer};
use services::herro_service::{HerroService, herro::herro_server::HerroServer};
use tonic::transport::Server;
use types::State;

mod middleware;
mod services;
mod types;

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
        .add_service(AdminServer::with_interceptor(admin_service, check_auth))
        .serve(address)
        .await?;

    Ok(())
}
