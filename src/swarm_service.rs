/*
*
* swarm_service.rs
*
*/

use crate::iot_manager;
use crate::message_store;


use crate::avlo::swarm_server::Swarm;
use crate::avlo::{IoTProcess, IoTDevice, DeviceGroup, IoTDeviceStatus, SwarmMessage};
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};

#[derive(Debug)]
struct SwarmService {
	all_messages: Vec<SwarmMessage>,
}


#[tonic::async_trait]
impl Swarm for SwarmService {

	type StartCommunicationStream = mpsc::Receiver<Result<SwarmMessage, Status>>;

	async fn join_swarm(&self, request: Request<IoTProcess>) -> Result<Response<IoTDevice>, Status> {
		let iot_device_ident = format!("iot-{}", iot_manager::IoTManager::singleton().get_count());
		let cur_proc = request.into_inner();
		let new_device = IoTDevice {
			device_id: iot_device_ident,
			owner: Some(cur_proc),
			neighbour: Vec::new()
		};
		let new_device_cpy = new_device.clone();
		let _res_manager = iot_manager::IoTManager::singleton_mut().add_new_device(new_device_cpy);
		Ok(Response::new(new_device))
	}

	async fn suspect_device(&self, request: Request<DeviceGroup>) -> Result<Response<IoTDeviceStatus>, Status> {
		let suspected_devices = request.into_inner().device_id;
		match iot_manager::IoTManager::singleton_mut().add_suspected_device(suspected_devices) {
			Err(_suspect_err) => {
				Err(Status::unknown("there was an error adding suspected devices to the monitoring list"))
			},
			Ok(_suspect_success) => {
				let device_status = IoTDeviceStatus {
					value: 1
				};
				Ok(Response::new(device_status))
			}
		}
	}

	async fn resurrect_device(&self, request: Request<DeviceGroup>) -> Result<Response<IoTDeviceStatus>, Status> {
		let to_be_resurrected = request.into_inner().device_id;
		match iot_manager::IoTManager::singleton_mut().resurrect_device(to_be_resurrected) {
			Err(failed_resurrection_err) => {
				Err(Status::unknown(failed_resurrection_err))
			},
			Ok(_resurrection_success) => {
				let device_status = IoTDeviceStatus {
					value: 0
				};
				Ok(Response::new(device_status))
			}
		}
	}

	async fn start_communication(&self, request: Request<()>) -> Result<Response<Self::StartCommunicationStream>, Status> {
		unimplemented!()
	}

	async fn deliver_message(&self, request: Request<SwarmMessage>) -> Result<Response<()>, Status> {
		let for_delivery = request.into_inner();
		// should mark the time the message was delivered

		match message_store::MessageStore::singleton_mut().add_message(&for_delivery) {
			Err(failed_delivery) => {Err(Status::unknown(failed_delivery))}
			Ok(_delivery_success) => {Ok(Response::new(()))}
		}
	}
}