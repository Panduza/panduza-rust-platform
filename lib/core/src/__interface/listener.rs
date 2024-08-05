use crate::TaskResult;
use crate::{__platform_error, link};
use crate::interface::AmInterface;
use super::subscriber::Subscriber;

/// Message handler
/// 
pub struct Listener {
    /// Shared state core
    interface: AmInterface,

    /// Subscriber
    subscriber: Box<dyn Subscriber>,

    /// Default link
    link: link::InterfaceHandle,
}

impl Listener {

    /// Create a new instance of the Listener
    /// 
    pub fn new(interface: AmInterface, subscriber: Box<dyn Subscriber>, link: link::InterfaceHandle)
        -> Listener {
        return Listener { interface, subscriber, link }
    }

    /// Task code that runs the interface Listener
    /// 
    /// move the listener into the task 
    /// 
    pub async fn run_task(mut self) -> TaskResult {
        loop {
            // Get a new message
            let msg = 
                self.link.rx().recv().await
                    .ok_or( __platform_error!("Listener channel closed") )?; // Critical error => need to stop

            // Process the message
            if let Err(e) = self.subscriber.process(&mut self.interface, &msg).await {
                self.interface.lock().await.set_event_error(e.to_string());
            }
        }
        Ok(())
    }

}

