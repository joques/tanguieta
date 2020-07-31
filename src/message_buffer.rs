/* 
*
* message_buffer.rs 
*
*/

use crate::avlo::SwarmMessage;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug)]
pub struct MessageBuffer<'a> {
	buf_store: Vec<&'a  SwarmMessage>,
}

lazy_static! {
    pub static ref MBUFF: RwLock<MessageBuffer<'static>> = RwLock::new(MessageBuffer::create());
}

impl<'a> MessageBuffer<'a> {
	pub fn create() -> Self {
		Self {
			buf_store: Vec::new()
		}
	}

	pub fn add_to_buffer(&mut self, new_msg: &'a SwarmMessage) -> Result<&str, &str> {
		self.buf_store.push(new_msg);
		Ok("new message added")
	}

	pub fn find_message(&self, message_id: String) -> Option<&SwarmMessage> {
		for cur_msg in self.buf_store.iter() {
			if cur_msg.swarm_ident == message_id {
				return Some(cur_msg)
			} else {
				continue;
			}
		}

		None
	}

	pub fn singleton() -> RwLockReadGuard<'static, MessageBuffer<'static>> {
		MBUFF.read().unwrap()
	}

	pub fn singleton_mut() -> RwLockWriteGuard<'static, MessageBuffer<'static>> {
		MBUFF.write().unwrap()
	}
}