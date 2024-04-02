

use crate::subscription;
use crate::interface::AmInterface;

use crate::connection::AmConnection;

use serde_json;
use tokio::sync::Mutex;

use crate::platform::{self, TaskPoolLoader};
use crate::platform::PlatformError;

use crate::device::traits::DeviceActions;

use crate::link;
use crate::link::AmManager as AmLinkManager;

/// A device manage a set of interfaces
/// 
pub struct Device {

    /// Device name
    dev_name: String,
    bench_name: String,


    started: bool,
    stoppable: bool,

    
    actions: Box<dyn DeviceActions>,

    interfaces: Vec<AmInterface>,

    /// Connection link manager
    /// To generate connection links for the interfaces
    connection_link_manager: AmLinkManager,
}

impl Device {

    /// Create a new instance of the Device
    /// 
    pub fn new<
        A: Into<String>,
        B: Into<String>,
    >
    (
        dev_name: A,
        bench_name: B,
        actions: Box<dyn DeviceActions>,

        connection_link_manager: AmLinkManager,
    ) -> Device 
    {
        // Create the object
        let obj = Device {
            dev_name: dev_name.into(),
            bench_name: bench_name.into(),

            started: false,
            stoppable: false,

            actions: actions,
            interfaces: Vec::new(),

            connection_link_manager: connection_link_manager
        };

        // Info log
        obj.log_info("Device created");

        // Return the object
        return obj;
    }

    /// Get the device name
    /// 
    #[inline]
    pub fn dev_name(&self) -> &String {
        return &self.dev_name;
    }

    /// Get the bench name
    /// 
    #[inline]
    pub fn bench_name(&self) -> &String {
        return &self.bench_name;
    }

    /// Attach default connection
    ///
    // async fn attach_default_connection(&mut self, interface: AmInterface) {

    //     let c = self.default_connection.as_ref().unwrap();
    //     let mut interface_lock = interface.lock().await;
    //     // let requests = interface_lock.subscription_requests().await;

    //     let topic = interface_lock.get_topic().await;
    //     let att_names = interface_lock.attributes_names().await;

    //     let mut requests = vec![
    //         subscription::Request::new( subscription::ID_PZA, "pza" ),
    //         subscription::Request::new( subscription::ID_PZA_CMDS_SET, &format!("pza/{}/cmds/set", topic) )
    //     ];

    //     for att_name in att_names {
    //         let request = subscription::Request::new( att_name.0, &format!("pza/{}/{}", topic, att_name.1) );
    //         requests.push(request);
    //     }

    //     let x: link::InterfaceHandle = c.lock().await.request_link(requests).await.unwrap();
    //     interface_lock.set_default_link(x).await;

    // }


    /// Create and Start the interfaces
    /// 
    pub async fn start_interfaces(&mut self, task_loader: &mut TaskPoolLoader) {
        // Do nothing if already started
        if self.started {
            return;
        }
        self.log_info("Start Interfaces...");

        // create interfaces
        self.interfaces = self.actions.create_interfaces(&serde_json::Value::Null);

        // Do nothing if no interface in the device
        if self.interfaces.len() == 0 {
            self.log_warn("No interface to start, skip device start");
            return;
        }

        let dev_name = self.dev_name().clone();
        let bench_name = self.bench_name().clone();



        let mut interfaces = self.interfaces.clone();
        for interface in interfaces.iter_mut() {
            let itf = interface.clone();

            // Set names
            itf.lock().await.set_dev_and_bench_names(dev_name.clone(), bench_name.clone()).await;



            itf.lock().await.start(task_loader).await;
        }

        // log
        self.log_info("Interfaces started !");
        self.started = true;
    }



    /// Log info
    /// 
    #[inline]
    pub fn log_info<A: Into<String>>(&self, text: A) {
        tracing::info!(class="Device", bname=self.bench_name, dname=self.dev_name, "{}", text.into());
    }

    /// Log warning
    /// 
    #[inline]
    pub fn log_warn<A: Into<String>>(&self, text: A) {
        tracing::warn!(class="Device", bname=self.bench_name, dname=self.dev_name, "{}", text.into());
    }

}


