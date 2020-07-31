#[macro_use]
extern crate lazy_static;

pub mod avlo {
	tonic::include_proto!("avlo");
}

mod swarm_service;

// mod iot_manager;

use avlo::swarm_server::{Swarm};
use avlo::{IoTProcess, IoTDevice, DeviceGroup, IoTDeviceStatus, SwarmMessage};
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};


fn main() {
    println!("Hello, world!");
}
