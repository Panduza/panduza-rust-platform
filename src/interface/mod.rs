
use std::sync::Arc;
use serde_json::Value;
use tokio::sync::Mutex;

use crate::subscription::Request as SubscriptionRequest;
use crate::connection::LinkInterfaceHandle;



use async_trait::async_trait;


pub mod fsm;
pub mod core;
pub mod listener;

use crate::interface::fsm::Fsm;
use crate::interface::core::{ AmCore, Core };
use crate::interface::listener::Listener;

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


/// 
pub struct Interface {
        
    /// Core Object
    core: AmCore,

    /// FSM
    fsm: Arc<Mutex<Fsm>>,

    /// Listener
    listener: Arc<Mutex<Listener>>,
}
pub type AmInterface = Arc<Mutex<Interface>>;


impl Interface {

    /// Create a new instance of the Interface
    /// 
    pub fn new<A: Into<String>, B: Into<String>, C: Into<String>>(
        name: A, dev_name: B, bench_name: C,
        idn: Box<dyn IdentityProvider>,
        states: Box<dyn fsm::States>,
        subscriber: Box<dyn listener::Subscriber>) -> AmInterface {


        let mut core = Arc::new(Mutex::new(Core::new(name, dev_name, bench_name)));
        
        // d.set_info(subscriber.get_info());


        return Arc::new(Mutex::new(
            Interface {
                core: core.clone(),
                fsm: Arc::new(Mutex::new(Fsm::new(core.clone(), states))),
                listener: Arc::new(Mutex::new(Listener::new(core.clone(), subscriber)))
            }
        ));
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
    pub async fn subscription_requests(&self) -> Vec<SubscriptionRequest> {
        return self.listener.lock().await.subscription_requests().await;
    }

    ///
    /// 
    pub async fn add_link(&mut self, link: LinkInterfaceHandle) {
        let mut listener = self.listener.lock().await;

        self.core.lock().await.add_client(link.client.clone());

        listener.add_link(link);


    }


    pub async fn set_name(&mut self, name: String) {
        self.core.lock().await.set_name(name);
    }

    pub async fn set_dev_name(&mut self, dev_name: String) {
        self.core.lock().await.set_dev_name(dev_name);
    }

    pub async fn set_bench_name(&mut self, bench_name: String) {
        self.core.lock().await.set_bench_name(bench_name);
    }



}

