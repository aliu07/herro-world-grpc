use herro::{HerroRequest, herro_client::HerroClient};
use std::io::stdin;
use tonic::metadata::{Ascii, MetadataValue};

use crate::admin::{GetCountRequest, admin_client::AdminClient};

pub mod herro {
    tonic::include_proto!("herro");
}

pub mod admin {
    tonic::include_proto!("admin");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut herro_client = HerroClient::connect("http://[::1]:8080").await?;
    let mut admin_client = AdminClient::connect("http://[::1]:8080").await?;

    loop {
        println!("\nPlease enter a command:");
        let mut cmd = String::new();
        stdin().read_line(&mut cmd).unwrap();
        let cmd = cmd.trim();

        match cmd {
            "quit" => break,
            "help" => {
                println!("Available commands:");
                println!("- quit");
                println!("- hello");
                println!("- count");
            }
            "herro" => {
                println!("\nPlease enter your name:");
                let mut name = String::new();
                stdin().read_line(&mut name).unwrap();
                let name = name.trim();

                // Service invocation
                let request = tonic::Request::new(HerroRequest {
                    name: String::from(name),
                });

                let response = herro_client.say_herro(request).await?;
                println!("Got '{}' from service!", response.into_inner().greeting);
            }
            "count" => {
                let mut request = tonic::Request::new(GetCountRequest {});

                // Header metadata
                let my_secret_token: MetadataValue<Ascii> = MetadataValue::from_static("123456");
                request
                    .metadata_mut()
                    .insert("authorization", my_secret_token);

                let response = admin_client.get_request_count(request).await?;
                println!("Got '{}' from service!", response.into_inner().count);
            }
            _ => {
                println!("Unkown command. Type 'help' for a list of commands.");
            }
        }
    }

    Ok(())
}
