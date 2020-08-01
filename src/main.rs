#[macro_use]
extern crate lazy_static;

pub mod avlo {
	tonic::include_proto!("avlo");
}

mod swarm_service;
mod iot_manager;
mod message_buffer;
mod message_store;

use avlo::swarm_server::{SwarmServer};
use tonic::transport::Server;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Swarm server...!");

    let addr = "[::1]:10000".parse().unwrap();

    // should load all messages right away
    let sm_service = swarm_service::SwarmService::new();

    let svc = SwarmServer::new(sm_service);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
