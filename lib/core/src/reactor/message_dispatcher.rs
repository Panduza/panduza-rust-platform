use std::sync::Weak;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use bytes::Bytes;

use crate::MessageHandler;

/// Data used by the core the dispatch input data
///
pub struct MessageDispatcher {
    /// List of attributes to trigger on message
    message_attributes: HashMap<String, Weak<Mutex<dyn MessageHandler>>>,
}

impl MessageDispatcher {
    /// Create a new MessageDispatcher
    ///
    pub fn new() -> Self {
        Self {
            message_attributes: HashMap::new(),
        }
    }

    pub fn register_message_attribute(
        &mut self,
        topic: String,
        attribute: Arc<Mutex<dyn MessageHandler>>,
    ) {
        self.message_attributes
            .insert(topic, Arc::downgrade(&attribute));
    }

    /// Trigger the on_message of the attribute
    ///
    pub async fn trigger_on_change(&self, topic: &str, new_value: &Bytes) {
        println!("{:?}", self.message_attributes.keys());
        if let Some(attribute) = self.message_attributes.get(topic) {
            println!("------");
            match attribute.upgrade() {
                Some(attribute) => {
                    println!("pok here");
                    attribute.lock().await.on_message(new_value).await;
                }
                None => {
                    println!("Attribute not found");
                }
            }
        }
    }
}
