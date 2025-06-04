use herro::{HerroRequest, herro_client::HerroClient};
use std::io::stdin;

pub mod herro {
    tonic::include_proto!("herro");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HerroClient::connect("http://[::1]:8080").await?;

    loop {
        println!("\nPlease enter your name:");
        let mut name = String::new();
        stdin().read_line(&mut name).unwrap();
        let name = name.trim();

        if name.to_lowercase() == "quit" {
            break;
        }

        // Service invocation
        let request = tonic::Request::new(HerroRequest {
            name: String::from(name),
        });

        let response = client.say_herro(request).await?;
        println!("Got '{}' from service!", response.into_inner().greeting);
    }

    Ok(())
}
