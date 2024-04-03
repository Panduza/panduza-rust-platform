use std::sync::Arc;

use futures::FutureExt;
use tokio::sync::Mutex;

use crate::{link::{self, InterfaceHandle}, platform::TaskPoolLoader, platform_error, subscription};

use super::{core::{AmCore, Core}, fsm::{self, Fsm}, listener::Listener, subscriber::Subscriber};

use crate::link::AmManager as AmLinkManager;

use super::builder::Builder as InterfaceBuilder;

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

    /// Build a new interface
    /// 
    pub async fn build<A: Into<String>, B: Into<String>>
        (builder: InterfaceBuilder, dev_name: A, bench_name: B, connection_link_manager: AmLinkManager) 
            -> AmInterface
    {
        let _dev_name = dev_name.into();
        let _bench_name = bench_name.into();

        // Topic name
        let topic = format!("pza/{}/{}/{}", _dev_name, _bench_name, builder.name);

        // Get attributes names
        let att_names = builder.subscriber.attributes_names().await;

        // Build subscriptions requests
        let mut requests = vec![
            subscription::Request::new( subscription::ID_PZA, "pza" ),
            subscription::Request::new( subscription::ID_PZA_CMDS_SET, &format!("{}/cmds/set", topic) )
        ];        
        for att_name in att_names {
            let request = subscription::Request::new( att_name.0, &format!("{}/{}", topic, att_name.1) );
            requests.push(request);
        }

        // Create the link with the connection
        let link = connection_link_manager.lock().await.request_link(requests).await.unwrap();
        
        // Create the interface
        return Interface::new_am(
            builder.name,
            _dev_name,
            _bench_name,
            builder.itype,
            builder.version,
            builder.states,
            builder.subscriber,
            link
        );
    }

    /// Create a new instance of the Interface
    /// 
    pub fn new<A: Into<String>, B: Into<String>, C: Into<String>, D: Into<String>, E: Into<String>>(
        name: A,
        dev_name: B,
        bench_name: C,

        itype: D,
        version: E,

        states: Box<dyn fsm::States>,
        subscriber: Box<dyn Subscriber>,
        
        link: link::InterfaceHandle,
        ) -> Interface
    {
        let core_obj = Core::new(name, dev_name, bench_name, link.client());
        let core = Arc::new(Mutex::new( core_obj ));
        return 
            Interface {
                core: core.clone(),
                fsm: Arc::new(Mutex::new(Fsm::new(core.clone(), states ))),
                listener: Listener::new_am(core.clone(), subscriber, link)
            }
        ;
    }

    /// Create a new instance of the Interface
    /// 
    pub fn new_am<A: Into<String>, B: Into<String>, C: Into<String>, D: Into<String>, E: Into<String>>(
        name: A,
        dev_name: B,
        bench_name: C,

        itype: D,
        version: E,

        states: Box<dyn fsm::States>,
        subscriber: Box<dyn Subscriber>,
        
        link: link::InterfaceHandle,
        ) -> AmInterface
    {
        return Arc::new(Mutex::new(
            Interface::new(name, dev_name, bench_name, itype, version, states, subscriber, link)
        ));
    }

    /// Start the interface, run it into tasks
    /// 
    pub async fn start(&mut self, task_loader: &mut TaskPoolLoader) {
        
        let fsm = self.fsm.clone();
        let listener = self.listener.clone();

        // FSM Task
        task_loader.load(async move {
            loop {
                fsm.lock().await.run_once().await;
            }
        }.boxed()).unwrap();

        // Listener Task
        // Ensure communication with the MQTT connection
        let interface_name = self.core.lock().await.name().clone() ;
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

        // Log success
        self.core.lock().await.log_info("Interface started");
    }



    



    

    /// Build the base topic of the interface
    ///
    pub async fn get_topic(&self) -> String {
        let core_lock = self.core.lock().await;
        return format!("pza/{}/{}/{}",
            core_lock.bench_name(),
            core_lock.dev_name(),
            core_lock.name());
    }


}

