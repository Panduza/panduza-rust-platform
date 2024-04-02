use std::sync::Arc;

use rumqttc::AsyncClient;
use tokio::sync::{mpsc, Mutex};

use crate::subscription;

/// Link handle for the interface
/// 
pub struct InterfaceHandle
{
    /// Mqtt client
    pub client: AsyncClient,

    /// Channel to receive messages from the connection
    pub rx: mpsc::Receiver<subscription::Message>
}

impl InterfaceHandle {

    pub fn new(client: AsyncClient, rx: mpsc::Receiver<subscription::Message>) -> InterfaceHandle {
        return InterfaceHandle {
            client: client.clone(),
            rx: rx
        }
    }

    pub fn get_client(&self) -> AsyncClient {
        return self.client.clone();
    }

}
