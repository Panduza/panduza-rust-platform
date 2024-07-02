
use crate::{__platform_error, link};

use crate::interface::AmInterface;

use super::subscriber::Subscriber;

use crate::TaskResult;


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
    pub fn new(interface: AmInterface, subscriber: Box<dyn Subscriber>, link: link::InterfaceHandle) -> Listener {
        return Listener {
            interface,
            subscriber,
            link
        }
    }

    /// Task code that runs the interface Listener
    /// 
    /// move the listener into the task 
    /// 
    pub async fn run_task(mut self) -> TaskResult {

        loop {
            let msg = 
                self.link.rx().recv().await
                    .ok_or( __platform_error!("Listener channel closed") )?; // Critical error => need to stop
                    

            self.subscriber.process(&mut self.interface, &msg).await?; // error warning need to live again

        }

        Ok(())
    }


}

