/* 
*
* message_store.rs 
*
*/

use crate::avlo::SwarmMessage;
use std::collections::HashMap;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug)]
pub struct MessageStore<'a> {
	store: HashMap<String, Vec<&'a SwarmMessage>>,
}

lazy_static! {
    pub static ref MSTORE: RwLock<MessageStore<'static>> = RwLock::new(MessageStore::create());
}

impl<'a> MessageStore<'a> {
	pub fn create() -> Self {
		Self {
			store: HashMap::new()
		}
	}

	pub fn add_message(&mut self, msg: &'a SwarmMessage) -> Result<&str, &str> {
		let the_topic = msg.topic;
		match self.store.get_mut(the_topic) {
			None => {
				let mut new_topic_store = Vec::new();
				new_topic_store.push(msg);
				self.store.insert(the_topic, new_topic_store)
				Ok("Message added for brand new topic")
			}
			Some(mesage_col) => {
				if messages.iter().any(|cur_msg| cur_msg.swarm_ident == msg.swarm_ident) {
					Err("The message has already been delivered")
				} else {
					messages.push(msg);
					Ok("New message added for topic")
				}
			}
		}
	}

	pub fn singleton() -> RwLockReadGuard<'static, MessageStore<'static>> {
		MSTORE.read().unwrap()
	}

	pub fn singleton_mut() -> RwLockWriteGuard<'static, MessageStore<'static>> {
		MSTORE.write().unwrap()
	}
}