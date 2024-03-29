
use crate::connection::AmLinkConnectionManager;
use crate::{builtin_devices, platform_error, subscription};
use crate::interface::AmInterface;

use crate::connection::AmConnection;
use crate::connection::LinkInterfaceHandle;

use serde_json;
use tokio::task::JoinSet;
use tokio::sync::Mutex;

use crate::platform::{self, TaskPoolLoader};
use crate::platform::PlatformError;

use crate::device::traits::DeviceActions;


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



    /// Default connection
    default_connection: Option<AmLinkConnectionManager>,

    /// Operational connection
    operational_connection: Option<AmLinkConnectionManager>
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
        actions: Box<dyn DeviceActions>
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

            default_connection: None,
            operational_connection: None
        };

        // Info log
        tracing::info!(class="Device", bname=obj.bench_name, dname=obj.dev_name, "Device created");

        // Return the object
        return obj;
    }



    pub fn get_name(&self) -> &String {
        return &self.dev_name;
    }

    pub fn set_bench_name(&mut self, bench_name: String) {
        self.bench_name = bench_name;
    }
    pub fn get_bench_name(&self) -> &String {
        return &self.bench_name;
    }

    /// Attach default connection
    ///
    async fn attach_default_connection(&mut self, interface: AmInterface) {

        if self.default_connection.is_some() {
            let c = self.default_connection.as_ref().unwrap();
            let mut interface_lock = interface.lock().await;
            // let requests = interface_lock.subscription_requests().await;

            let topic = interface_lock.get_topic().await;
            let att_names = interface_lock.attributes_names().await;

            let mut requests = vec![
                subscription::Request::new( subscription::ID_PZA, "pza" ),
                subscription::Request::new( subscription::ID_PZA_CMDS_SET, &format!("pza/{}/cmds/set", topic) )
            ];

            for att_name in att_names {
                let request = subscription::Request::new( att_name.0, &format!("pza/{}/{}", topic, att_name.1) );
                requests.push(request);
            }

            let x: LinkInterfaceHandle = c.lock().await.request_link(requests).await.unwrap();
            interface_lock.set_default_link(x).await;
        }
    }


    /// Create and Start the interfaces
    /// 
    pub async fn start_interfaces(&mut self, task_loader: &mut TaskPoolLoader) {
        // Do nothing if already started
        if self.started {
            return;
        }
        tracing::info!(class="Device", bname=self.bench_name, dname=self.dev_name,
            "Start Interfaces...");

        // create interfaces
        self.interfaces = self.actions.create_interfaces(&serde_json::Value::Null);

        // Do nothing if no interface in the device
        if self.interfaces.len() == 0 {
            tracing::warn!(class="Device", bname=self.bench_name, dname=self.dev_name,
                "No interface to start, skip device start");
            return;
        }

        let dev_name = self.get_name().clone();
        let bench_name = self.get_bench_name().clone();



        let mut interfaces = self.interfaces.clone();
        for interface in interfaces.iter_mut() {
            let itf = interface.clone();

            // Set names
            itf.lock().await.set_dev_and_bench_names(dev_name.clone(), bench_name.clone()).await;


            self.attach_default_connection(itf.clone()).await;

            itf.lock().await.start(task_loader).await;
        }

        // log
        tracing::info!(class="Device", bname=self.bench_name, dname=self.dev_name,
            "Interfaces started !");
        self.started = true;
    }

    /// Set default connection
    pub async fn set_default_connection(&mut self, connection: AmConnection) {
        self.default_connection = Some(connection.lock().await.clone_link_manager());
    }

    /// Set operational connection
    pub async fn set_operational_connection(&mut self, connection: AmConnection) {
        self.operational_connection = Some(connection.lock().await.clone_link_manager());
    }

}


