use std::sync::Arc;
use tokio::sync::Mutex;

use crate::link;
use crate::platform::PlatformError;
use crate::interface::core::AmCore;

use super::subscriber::Subscriber;

/// Message handler
/// 
pub struct Listener {
    /// Shared state core
    core: AmCore,

    /// Subscriber
    subscriber: Box<dyn Subscriber>,

    /// Default link
    link: link::InterfaceHandle,
}

impl Listener {
    
    /// Create a new instance of the Listener
    /// 
    pub fn new(core: AmCore, subscriber: Box<dyn Subscriber>, link: link::InterfaceHandle) -> Listener {
        return Listener {
            core,
            subscriber,
            link
        }
    }

    /// New instance inside a safe pointer
    /// 
    pub fn new_am(core: AmCore, subscriber: Box<dyn Subscriber>, link: link::InterfaceHandle) -> Arc<Mutex<Listener>> {
        return Arc::new(Mutex::new(Listener::new(core, subscriber, link)));
    }

    /// Run the listener once
    ///
    pub async fn run_once(&mut self) -> Result<(), PlatformError> {
        let msg = self.link.rx().recv().await;
        match msg {
            Some(msg) => {
                self.subscriber.process(&self.core, &msg).await;
            },
            None => {
                // do nothing
            }
        }
        Ok(())
    }

}
