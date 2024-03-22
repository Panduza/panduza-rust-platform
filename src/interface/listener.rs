use std::sync::Arc;
use futures::future::select_all;
use async_trait::async_trait;
use futures::FutureExt;
use tokio::task::JoinSet;
use crate::device::ConnectionUsagePolicy;
use crate::subscription;
use crate::interface::core::AmCore;
use crate::connection::LinkInterfaceHandle;

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

#[async_trait]
pub trait Subscriber : Send + Sync {

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

    /// Subscriber
    subscriber: Box<dyn Subscriber>,

    /// Default link
    default_link: Option<LinkInterfaceHandle>,

    /// Operational link
    operational_link: Option<LinkInterfaceHandle>,

    /// Connection usage policy
    connection_usage_policy: ConnectionUsagePolicy
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
            connection_usage_policy: ConnectionUsagePolicy::UseOperationalOnly
        }
    }

    ///
    ///
    pub async fn subscription_requests(&self) -> Vec<subscription::Request> {
        return self.subscriber.subscription_requests().await;
    }

    /// Set the default link
    ///
    pub fn set_default_link(&mut self, link: LinkInterfaceHandle) {
        self.default_link = Some(link);
    }

    /// Set the operational link
    /// 
    pub fn set_operational_link(&mut self, link: LinkInterfaceHandle) {
        self.operational_link = Some(link);
    }

    ///
    ///
    pub async fn run_once(&mut self) {


        // let tasks = JoinSet::new();


        let (result, _index, remaining_futures) = select_all(
            vec![
                self.default_link.as_mut().unwrap().rx.recv().boxed(),
                self.operational_link.as_mut().unwrap().rx.recv().boxed()
            ]
        ).await;
   
        println!("result {:?}", result);

        // for link in self.links.iter_mut() {
        //     let msg = link.rx.recv().await;
        //     match msg {
        //         Some(msg) => {
        //             self.subscriber.process(&self.core, &msg).await;
        //         },
        //         None => {
        //             // do nothing
        //         }
        //     }
        // }
    }

}
