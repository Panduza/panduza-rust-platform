use std::collections::LinkedList;
use std::sync::Arc;
use rumqttc::AsyncClient;
use serde_json::Value;
use tokio::sync::Mutex;

use crate::subscription::Request as SubscriptionRequest;
use crate::connection::LinkInterfaceHandle;

use crate::subscription::Message as SubscriptionMessage;


use async_trait::async_trait;


mod fsm;
mod core;
mod listener;


use crate::interface::core::Core;



// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------


#[async_trait]
pub trait IdentityProvider : Send {
    fn get_info(&self) -> Value;
}



// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------


pub struct Interface {
        
    /// Shared state data
    data: SharedData,

    fsm: Arc<Mutex<Fsm>>,
    listener: Arc<Mutex<Listener>>,
}
pub type SafeInterface = Arc<Mutex<Interface>>;


impl Interface {

    /// Create a new instance of the Interface
    /// 
    pub fn new(
        name: &str,
        state_impls: Box<dyn StateImplementations>, listener_impls: Box<dyn HandlerImplementations>) -> Interface {


        let mut d = Data::new();
        d.set_name(name.to_string());
        d.set_info(listener_impls.get_info());

        let data = Arc::new(Mutex::new(d));
        return Interface {
            data: data.clone(),
            fsm: Arc::new(Mutex::new(Fsm::new(data.clone(), state_impls))),
            listener: Arc::new(Mutex::new(Listener::new(data.clone(), listener_impls)))
        }
    }

    /// Start the interface, run it into tasks
    /// 
    pub async fn start(&mut self, task_pool: &mut tokio::task::JoinSet<()>) {
        
        let fsm = self.fsm.clone();
        let listener = self.listener.clone();

        task_pool.spawn(async move {
            loop {
                fsm.lock().await.run_once().await;
            }
        });

        task_pool.spawn(async move {
            loop {
                listener.lock().await.run_once().await;
            }
        });

    }

    ///
    /// 
    pub async fn get_subscription_requests(&self) -> Vec<SubscriptionRequest> {
        return self.listener.lock().await.get_subscription_requests().await;
    }

    ///
    /// 
    pub async fn add_link(&mut self, link: LinkInterfaceHandle) {
        let mut listener = self.listener.lock().await;

        self.data.lock().await.add_client(link.client.clone());

        listener.add_link(link);


    }


    pub async fn set_name(&mut self, name: String) {
        self.data.lock().await.set_name(name);
    }

    pub async fn set_dev_name(&mut self, dev_name: String) {
        self.data.lock().await.set_dev_name(dev_name);
    }

    pub async fn set_bench_name(&mut self, bench_name: String) {
        self.data.lock().await.set_bench_name(bench_name);
    }



}

