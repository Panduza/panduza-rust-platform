use std::sync::Arc;

use futures::FutureExt;
use tokio::sync::Mutex;

use crate::{link::{self}, platform::TaskPoolLoader, __platform_error_result, subscription};

use super::{fsm::{self, Fsm}, listener::Listener, subscriber::Subscriber, AmInterface, Interface};

use crate::link::AmManager as AmLinkManager;

use super::builder::Builder as InterfaceBuilder;

/// 
pub struct Runner {
    /// Core Object
    interface: AmInterface,

    /// FSM
    fsm: Arc<Mutex<Fsm>>,

    /// Listener
    listener: Arc<Mutex<Listener>>,
}
pub type AmRunner = Arc<Mutex<Runner>>;

impl Runner {

    /// Build a new interface
    /// 
    pub async fn build<A: Into<String>, B: Into<String>>
        (builder: InterfaceBuilder, dev_name: A, bench_name: B, connection_link_manager: AmLinkManager,
            platform_services: crate::platform::services::AmServices) 
            -> AmRunner
    {
        let _dev_name = dev_name.into();
        let _bench_name = bench_name.into();

        // Topic name
        let topic = format!("pza/{}/{}/{}", _bench_name, _dev_name, builder.name);

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
        return Runner::new_am(
            builder.name,
            _dev_name,
            _bench_name,
            builder.itype,
            builder.version,
            builder.states,
            builder.subscriber,
            link,
            platform_services
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
        platform_services: crate::platform::services::AmServices
        ) -> Runner
    {
        let core_obj = Interface::new_am(name, dev_name, bench_name, itype, version, link.client(), platform_services);
        return 
            Runner {
                interface: core_obj.clone(),
                fsm: Arc::new(Mutex::new(Fsm::new(core_obj.clone(), states ))),
                listener: Listener::new_am(core_obj.clone(), subscriber, link)
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
        platform_services: crate::platform::services::AmServices
        ) -> AmRunner
    {
        return Arc::new(Mutex::new(
            Runner::new(name, dev_name, bench_name, itype, version, states, subscriber, link, platform_services)
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
        let interface_name = self.interface.lock().await.name().clone() ;
        task_loader.load(async move {
            loop {
                if let Err(_) = listener.lock().await.run_once().await {
                    return __platform_error_result!(
                        format!("Interface {:?} Listen Task Error", interface_name)
                    );
                }
            }
        }.boxed()).unwrap();

        // Log success
        self.interface.lock().await.log_info("Interface started");
    }



    



    

    /// Build the base topic of the interface
    ///
    pub async fn _get_topic(&self) -> String {
        let core_lock = self.interface.lock().await;
        return format!("pza/{}/{}/{}",
            core_lock._bench_name(),
            core_lock._dev_name(),
            core_lock.name());
    }


}

