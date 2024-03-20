use async_trait::async_trait;
use crate::subscription;
use crate::interface::core::AmCore;
use crate::connection::LinkInterfaceHandle;

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

#[async_trait]
pub trait Subscriber : Send {

    /// Get the subscription requests
    async fn subscription_requests(&self) -> Vec<subscription::Request>;

    /// Process a message
    async fn process(&self, core: &AmCore, msg: &subscription::Message);

}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Message handler
/// 
pub struct Listener {
    /// Shared state core
    core: AmCore,

    /// 
    subscriber: Box<dyn Subscriber>,
    
    /// Default link
    default_link: Option<LinkInterfaceHandle>,

    /// Operational link
    operational_link: Option<LinkInterfaceHandle>,
}

impl Listener {
    
    /// Create a new instance of the Listener
    /// 
    pub fn new(core: AmCore, subscriber: Box<dyn Subscriber>) -> Listener {
        return Listener {
            core: core,
            subscriber: subscriber,
            default_link: None,
            operational_link: None,
        }
    }

    ///
    ///
    pub async fn subscription_requests(&self) -> Vec<subscription::Request> {
        return self.impls.subscription_requests().await;
    }

    ///
    /// 
    pub fn add_link(&mut self, link: LinkInterfaceHandle) {
        self.links.push_back(link);
    }
    
    ///
    ///
    pub async fn run_once(&mut self) {
        for link in self.links.iter_mut() {
            let msg = link.rx.recv().await;
            match msg {
                Some(msg) => {
                    self.impls.process(&self.core, &msg).await;
                },
                None => {
                    // do nothing
                }
            }
        }
    }

}
