/* 
*
* message_store.rs 
*
*/

use crate::avlo::SwarmMessage;
use std::collections::HashMap;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug)]
pub struct MessageStore {
	store: HashMap<String, Vec<SwarmMessage>>,
}

lazy_static! {
    pub static ref MSTORE: RwLock<MessageStore> = RwLock::new(MessageStore::create());
}

impl MessageStore {
	pub fn create() -> Self {
		Self {
			store: HashMap::new()
		}
	}

	pub fn add_message(&mut self, the_topic: String,  msg: SwarmMessage) -> Result<&str, &str> {
		
		match self.store.get_mut(&the_topic) {
			None => {
				let mut new_topic_store = Vec::new();
				new_topic_store.push(msg);
				self.store.insert(the_topic, new_topic_store);
				Ok("Message added for brand new topic")
			}
			Some(mesage_col) => {
				if mesage_col.iter().any(|cur_msg| cur_msg.swarm_ident == msg.swarm_ident) {
					Err("The message has already been delivered")
				} else {
					mesage_col.push(msg);
					Ok("New message added for topic")
				}
			}
		}
	}

	pub fn singleton() -> RwLockReadGuard<'static, MessageStore> {
		MSTORE.read().unwrap()
	}

	pub fn singleton_mut() -> RwLockWriteGuard<'static, MessageStore> {
		MSTORE.write().unwrap()
	}
}