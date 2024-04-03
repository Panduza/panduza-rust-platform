use std::sync::Arc;
use futures::future::select_all;
use async_trait::async_trait;
use futures::FutureExt;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use crate::platform::PlatformError;
use crate::subscription;
use crate::interface::core::AmCore;
use crate::link;
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

    /// 
    ///
    pub async fn attributes_names(&self) -> Vec<(subscription::Id, String)> {
        return self.subscriber.attributes_names().await;
    }


    /// Run the listener once
    ///
    pub async fn run_once(&mut self) -> Result<(), PlatformError> {


        // 
        let mut vv: Vec<std::pin::Pin<Box<dyn Future<Output = Option<subscription::Message>> + Send>>> = vec![];

        
        // match self.default_link {
        //     Some(_) => {
        //         vv.push(self.default_link.as_mut().unwrap().rx.recv().boxed());
        //     },
        //     None => {
        //         return platform_error!("No default link set", None);
        //     }
        // }


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
