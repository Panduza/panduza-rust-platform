use std::sync::Arc;
use futures::future::select_all;
use async_trait::async_trait;
use futures::FutureExt;
use tokio::task::JoinSet;
use crate::device::ConnectionUsagePolicy;
use crate::platform::PlatformError;
use crate::subscription;
use crate::interface::core::AmCore;
use crate::connection::LinkInterfaceHandle;
use futures::future::BoxFuture;
use futures::Future;


use crate::platform_error;

use super::subscriber::Subscriber;



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
    pub async fn attributes_names(&self) -> Vec<(subscription::Id, String)> {
        return self.subscriber.attributes_names().await;
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

    /// Run the listener once
    ///
    pub async fn run_once(&mut self) -> Result<(), PlatformError> {


        // 
        let mut vv: Vec<std::pin::Pin<Box<dyn Future<Output = Option<subscription::Message>> + Send>>> = vec![];

        

        // 
        match self.connection_usage_policy {
            ConnectionUsagePolicy::UseBoth => {
                match self.default_link {
                    Some(_) => {
                        vv.push(self.default_link.as_mut().unwrap().rx.recv().boxed());
                    },
                    None => {
                        return platform_error!("No default link set", None);
                    }
                }
                match self.operational_link {
                    Some(_) => {
                        vv.push(self.operational_link.as_mut().unwrap().rx.recv().boxed());
                    },
                    None => {
                        return platform_error!("No operational link set", None);
                    }
                }
            },
            ConnectionUsagePolicy::UseDefaultOnly => {
                match self.default_link {
                    Some(_) => {
                        vv.push(self.default_link.as_mut().unwrap().rx.recv().boxed());
                    },
                    None => {
                        return platform_error!("No default link set", None);
                    }
                }
            },
            ConnectionUsagePolicy::UseOperationalOnly => {
                match self.operational_link {
                    Some(_) => {
                        vv.push(self.operational_link.as_mut().unwrap().rx.recv().boxed());
                    },
                    None => {
                        return platform_error!("No operational link set", None);
                    }
                }
            },
        }


        // = vec![
        //     self.default_link.as_mut().unwrap().rx.recv().boxed(),
        //     self.operational_link.as_mut().unwrap().rx.recv().boxed()
        // ];

        let (msg, _index, remaining_futures) = select_all(
            vv
        ).await;

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
