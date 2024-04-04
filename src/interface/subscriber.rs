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

