use std::sync::Arc;

use futures::FutureExt;
use tokio::sync::Mutex;

use crate::{link, platform::TaskPoolLoader, platform_error, subscription};

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


    pub fn new<A: Into<String>, B: Into<String>>(
        dev_name: A,
        bench_name: B,
        builder: InterfaceBuilder,
        connection_link_manager: AmLinkManager,
        ) -> Interface
    {

        // builder have the subscriber to get atts names

        let att_names = builder.subscriber.attributes_names().await;


        let mut requests = vec![
            subscription::Request::new( subscription::ID_PZA, "pza" ),
            subscription::Request::new( subscription::ID_PZA_CMDS_SET, &format!("pza/{}/cmds/set", topic) )
        ];

    //     for att_name in att_names {
    //         let request = subscription::Request::new( att_name.0, &format!("pza/{}/{}", topic, att_name.1) );
    //         requests.push(request);
    //     }

    //     let x: link::InterfaceHandle = c.lock().await.request_link(requests).await.unwrap();
    //     interface_lock.set_default_link(x).await;



        let core_obj = Core::new(builder.name, dev_name, bench_name);
        let core = Arc::new(Mutex::new( core_obj ));
        return 
            Interface {
                core: core.clone(),
                fsm: Arc::new(Mutex::new(Fsm::new(core.clone(), builder.states))),
                listener: Listener::new_am(core.clone(), builder.subscriber, )
            }
        ;
    }



    // link: link::InterfaceHandle

    /// Create a new instance of the Interface
    /// 
    pub fn new_am<A: Into<String>, B: Into<String>>(
        dev_name: A,
        bench_name: B,
        builder: InterfaceBuilder
        ) -> AmInterface
    {
        return Arc::new(Mutex::new(
            Interface::new(dev_name, bench_name, builder);
        ));
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

    /// Get the name of the attributes managed by the interface
    /// 
    pub async fn attributes_names(&self) -> Vec<(subscription::Id, String)> {
        return self.listener.lock().await.attributes_names().await;
    }

    /// Set the default link
    ///
    pub async fn set_default_link(&mut self, link: link::InterfaceHandle) {
        let mut listener = self.listener.lock().await;
        self.core.lock().await.set_default_client(link.client.clone());
        listener.set_default_link(link);
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
            core_lock.bench_name(),
            core_lock.dev_name(),
            core_lock.name());
    }


}

