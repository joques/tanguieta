/*
* iot_manager.rs
*
* implementattion of device managers
*
*/

use crate::avlo::IoTDevice;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};


#[derive(Debug)]
pub struct IoTManager {
	count: u32,
	active_device_idents: Vec<String>,
	created_devices: Vec<IoTDevice>,
	suspected_iot_idents: Vec<String>,
	failed_iot_idents: Vec<String>,
}

lazy_static! {
    pub static ref THE_MAN: RwLock<IoTManager> = RwLock::new(IoTManager::create());
}


impl IoTManager {

	pub fn create() -> Self {
		Self {
			count: 0,
			active_device_idents: Vec::new(),
			created_devices: Vec::new(),
			suspected_iot_idents: Vec::new(),
			failed_iot_idents: Vec::new(),
		}
	}

	pub fn get_count(&self) -> u32 {
		self.count
	}

	pub fn add_new_device(&mut self, new_iot: IoTDevice) -> Result<&str, &str> {
		let new_device_id = new_iot.device_id.clone();
		self.active_device_idents.push(new_device_id);
		self.created_devices.push(new_iot);
		self.count += 1;
		Ok("new device added")
	}

	pub fn add_suspected_device(&mut self, suspected: Vec<String>) -> Result<&str, &str> {
		for single_suspect in suspected {
			if !self.suspected_iot_idents.iter().any(|cur_suspect| *cur_suspect == single_suspect) {
				self.suspected_iot_idents.push(single_suspect);
			}
		}
		Ok("suspect added")
	}

	pub fn resurrect_device(&mut self, candidates: Vec<String>) -> Result<&str, &str> {
		for single_candiate in candidates {
			if self.has_failed(&single_candiate) {
				return Err("Error! Cannot resurrect a failed iot");
			} else if self.suspected_iot_idents.iter().any(|cur_suspect| *cur_suspect == single_candiate) {
				self.suspected_iot_idents.retain(|cur_iot_ident| *cur_iot_ident != single_candiate);
			}
		}
		Ok("Old suspects resurrected")
	}

	fn has_failed(&self, iot_ident: &String) -> bool {
		self.failed_iot_idents.iter().any(|cur_iot_ident| *cur_iot_ident == *iot_ident)
	}

	pub fn singleton() -> RwLockReadGuard<'static, IoTManager> {
        // let m: RwLockReadGuard<'static, IoTManager> = THE_MAN.read().unwrap();
        // m
        THE_MAN.read().unwrap()
    }

    pub fn singleton_mut() -> RwLockWriteGuard<'static, IoTManager> {
    	THE_MAN.write().unwrap()	
    }
}