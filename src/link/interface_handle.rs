use std::sync::Arc;

use rumqttc::AsyncClient;
use tokio::sync::{mpsc, Mutex};

use crate::subscription;

/// Link handle for the interface
/// 
pub struct InterfaceHandle
{
    /// Mqtt client
    client: AsyncClient,

    /// Channel to receive messages from the connection
    rx: mpsc::Receiver<subscription::Message>
}

impl InterfaceHandle {

    /// Create a new instance of the InterfaceHandle
    /// 
    pub fn new(client: AsyncClient, rx: mpsc::Receiver<subscription::Message>) -> InterfaceHandle {
        return InterfaceHandle {
            client: client.clone(),
            rx: rx
        }
    }

    /// Clone the client
    /// 
    pub fn client(&self) -> AsyncClient {
        return self.client.clone();
    }

    /// Clone the receiver
    /// 
    pub fn rx(&mut self) -> &mut mpsc::Receiver<subscription::Message> {
        return &mut self.rx;
    }

}
