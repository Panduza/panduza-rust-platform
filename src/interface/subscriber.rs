use async_trait::async_trait;

use super::core::AmCore;
use crate::subscription;

/// Subscriber trait, to allow a user to insert its own processing of the messages
/// 
#[async_trait]
pub trait Subscriber : Send + Sync {

    // /// Get the subscription requests
    // async fn subscription_requests(&self) -> Vec<subscription::Request>;

    /// Get the subscription requests
    async fn attributes_names(&self) -> Vec<(subscription::Id, String)>;
    
    /// Process a message
    async fn process(&self, core: &AmCore, msg: &subscription::Message);

}

/// Process a message with common behaviour for all interface
/// 
pub async fn process_common(core: &AmCore, msg: &subscription::Message) {
    match msg {
        subscription::Message::ConnectionStatus (status) => {
            if status.connected {
                core.lock().await.set_event_connection_up();
            }
            else {
                core.lock().await.set_event_connection_down();
            }
        },
        subscription::Message::Mqtt(msg) => {
            match msg.get_id() {
                subscription::ID_PZA => {
                    core.lock().await.publish_info().await;

                    tracing::trace!("Ackk !!!");
                },
                _ => {
                    // not managed by the common level
                }
            }
        }
    }
}


