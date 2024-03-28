use std::mem::zeroed;
use std::sync::Arc;

use std::collections::{HashMap, LinkedList};

// use tokio::{task::yield_now, time::{sleep, Duration}};

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


mod factory;
mod manager;


pub type Factory = factory::Factory;
pub type Manager = manager::Manager;
pub type AmManager = manager::AmManager;


/// Defines the policy for using the 2 connections (default & operational)
///
#[derive(Clone)]
pub enum ConnectionUsagePolicy {
    /// the device must use both connections if possible
    UseBoth,
    /// the device must use only the default connection
    UseDefaultOnly,
    /// the device must use only the operational connection
    UseOperationalOnly
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------


pub trait Producer : Send {
    fn create_device(&self) -> Result<Device, String>;
}


// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

pub trait DeviceActions : Send {

    // fn hunt(&self) -> LinkedList<serde_json::Value>;

    /// Create a new instance of the Device
    /// 
    fn create_interfaces(&self, device_settings: &serde_json::Value) -> Vec<AmInterface>;

}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

pub struct Device {

    /// Device name
    name: String,
    bench_name: String,


    started: bool,
    stoppable: bool,

    
    actions: Box<dyn DeviceActions>,

    interfaces: Vec<AmInterface>,


    // Connection Usage Policy
    connection_usage_policy: ConnectionUsagePolicy,

    /// Default connection
    default_connection: Option<AmLinkConnectionManager>,

    /// Operational connection
    operational_connection: Option<AmLinkConnectionManager>
}

impl Device {

    /// Create a new instance of the Device
    /// 
    pub fn new(actions: Box<dyn DeviceActions>) -> Device {
        return Device {
            name: String::from("changeme"),
            bench_name: String::from("changeme"),

            started: false,
            stoppable: false,

            actions: actions,
            interfaces: Vec::new(),

            connection_usage_policy: ConnectionUsagePolicy::UseOperationalOnly,
            default_connection: None,
            operational_connection: None
        }
    }


    pub fn set_name(&mut self, name: String) {
        self.name = name;
        tracing::info!(class="Device", bname=self.bench_name, dname=self.name,
            "Device created");
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
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

    /// Attach operational connection
    ///
    async fn attach_operational_connection(&mut self, interface: AmInterface) {
        if self.operational_connection.is_some() {
            let c = self.operational_connection.as_ref().unwrap();
            let mut interface_lock = interface.lock().await;
            
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
            interface_lock.set_operational_link(x).await;
        }
    }

    /// Create and Start the interfaces
    /// 
    pub async fn start_interfaces(&mut self, task_loader: &mut TaskPoolLoader) {
        // Do nothing if already started
        if self.started {
            return;
        }
        tracing::info!(class="Device", bname=self.bench_name, dname=self.name,
            "Start Interfaces...");

        // create interfaces
        self.interfaces = self.actions.create_interfaces(&serde_json::Value::Null);

        // Do nothing if no interface in the device
        if self.interfaces.len() == 0 {
            tracing::warn!(class="Device", bname=self.bench_name, dname=self.name,
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
            // self.attach_operational_connection(itf.clone()).await;

            itf.lock().await.start(task_loader).await;
        }

        // log
        tracing::info!(class="Device", bname=self.bench_name, dname=self.name,
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


