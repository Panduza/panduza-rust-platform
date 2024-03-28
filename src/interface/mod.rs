use std::sync::Arc;
use futures::FutureExt;
use serde_json::Value;
use tokio::sync::Mutex;
use async_trait::async_trait;

use crate::platform_error;
use crate::platform::TaskPoolLoader;
use crate::device::ConnectionUsagePolicy;
use crate::subscription;
use crate::subscription::Request as SubscriptionRequest;
use crate::connection::LinkInterfaceHandle;

pub mod fsm;
pub mod core;
pub mod listener;
pub mod subscriber;

use crate::interface::fsm::Fsm;
use crate::interface::core::Core;
use crate::interface::core::AmCore;
use crate::interface::listener::Listener;


use crate::interface::subscriber::Subscriber;

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
    pub fn new<A: Into<String>>(
        name: A,
        idn: Box<dyn IdentityProvider>,
        states: Box<dyn fsm::States>,
        subscriber: Box<dyn Subscriber>) -> AmInterface {


        let mut core_obj = Core::new(name);
        core_obj.set_info(idn.get_info());

        let core = Arc::new(Mutex::new( core_obj ));

        return Arc::new(Mutex::new(
            Interface {
                core: core.clone(),
                fsm: Arc::new(Mutex::new(Fsm::new(core.clone(), states))),
                listener: Arc::new(Mutex::new(Listener::new(core.clone(), subscriber)))
            }
        ));
    }

    /// Set the dev and bench name to the interface
    /// 
    pub async fn set_dev_and_bench_names<A: Into<String>, B: Into<String>>(&mut self, dev_name: A, bench_name: B) {
        self.core.lock().await.set_dev_and_bench_names(dev_name, bench_name);
    }

    /// Start the interface, run it into tasks
    /// 
    pub async fn start(&mut self, task_loader: &mut TaskPoolLoader) {
        
        let fsm = self.fsm.clone();
        let listener = self.listener.clone();

        task_loader.load(async move {
            loop {
                fsm.lock().await.run_once().await;
            }
        }.boxed()).unwrap();


        // Listen Task
        let interface_name = self.core.lock().await.get_name().clone() ;
        task_loader.load(async move {
            loop {
                if let Err(e) = listener.lock().await.run_once().await {
                    return platform_error!(
                        format!("Interface {:?} Listen Task Error", interface_name)
                        , Some(Box::new(e))
                    );
                }
            }
        }.boxed()).unwrap();



        // Log
        {
            let bname = self.core.lock().await.get_bench_name().clone();
            let dname = self.core.lock().await.get_dev_name().clone();
            let iname = self.core.lock().await.get_name().clone() ;
            tracing::info!(class="Interface", bname=bname, dname=dname, iname=iname,
                "Interface started");
        }
    }

    /// Get the name of the attributes managed by the interface
    /// 
    pub async fn attributes_names(&self) -> Vec<(subscription::Id, String)> {
        return self.listener.lock().await.attributes_names().await;
    }

    /// Set the default link
    ///
    pub async fn set_default_link(&mut self, link: LinkInterfaceHandle) {
        let mut listener = self.listener.lock().await;
        self.core.lock().await.set_default_client(link.client.clone());
        listener.set_default_link(link);
    }

    /// Set the operational link
    ///
    pub async fn set_operational_link(&mut self, link: LinkInterfaceHandle) {
        let mut listener = self.listener.lock().await;
        self.core.lock().await.set_operational_client(link.client.clone());
        listener.set_operational_link(link);
    }

    // pub async fn set_name(&mut self, name: String) {
    //     self.core.lock().await.set_name(name);
    // }

    // pub async fn set_dev_name(&mut self, dev_name: String) {
    //     self.core.lock().await.set_dev_name(dev_name);
    // }

    // pub async fn set_bench_name(&mut self, bench_name: String) {
    //     self.core.lock().await.set_bench_name(bench_name);
    // }


    /// Build the base topic of the interface
    ///
    pub async fn get_topic(&self) -> String {
        let core_lock = self.core.lock().await;
        return format!("pza/{}/{}/{}",
            core_lock.get_bench_name(),
            core_lock.get_dev_name(),
            core_lock.get_name());
    }

    /// Set the connection usage policy
    /// 
    pub async fn set_connection_usage_policy(&mut self, policy: ConnectionUsagePolicy) {
        self.core.lock().await.set_connection_usage_policy(policy).await;
    }

}

