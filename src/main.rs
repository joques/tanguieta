#[macro_use]
extern crate lazy_static;

pub mod avlo {
	tonic::include_proto!("avlo");
}

mod swarm_service;
mod iot_manager;
mod message_buffer;
mod message_store;

use avlo::swarm_server::{Swarm};





fn main() {
    println!("Hello, world!");
}
