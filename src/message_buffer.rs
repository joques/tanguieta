/* 
*
* message_buffer.rs 
*
*/

use crate::avlo::SwarmMessage;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug)]
pub struct MessageBuffer {
	buf_store: Vec<SwarmMessage>,
}

lazy_static! {
    pub static ref MBUFF: RwLock<MessageBuffer> = RwLock::new(MessageBuffer::create());
}

impl MessageBuffer {
	pub fn create() -> Self {
		Self {
			buf_store: Vec::new()
		}
	}

	pub fn add_to_buffer(&mut self, new_msg: SwarmMessage) -> Result<&str, &str> {
		self.buf_store.push(new_msg);
		Ok("new message added")
	}

	pub fn find_message(&self, message_id: String) -> Option<SwarmMessage> {
		for cur_msg in self.buf_store.iter() {
			if cur_msg.swarm_ident == message_id {
				return Some(cur_msg.clone())
			} else {
				continue;
			}
		}

		None
	}

	pub fn singleton() -> RwLockReadGuard<'static, MessageBuffer> {
		MBUFF.read().unwrap()
	}

	pub fn singleton_mut() -> RwLockWriteGuard<'static, MessageBuffer> {
		MBUFF.write().unwrap()
	}
}