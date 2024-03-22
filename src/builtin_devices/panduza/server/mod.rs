use async_trait::async_trait;
use tokio::time::{sleep, Duration};

use crate::subscription;
use crate::interface::{self, Interface};
use crate::interface::core::AmCore;
use crate::interface::AmInterface;
use crate::device::{ Device, DeviceActions, Producer };

struct TestInterfaceListener;

#[async_trait]
impl interface::listener::Subscriber for TestInterfaceListener {

    /// List of subscription requests
    ///
    async fn subscription_requests(&self) -> Vec<subscription::Request> {
        return vec![
            subscription::Request::new( 0, "pza" )
        ];
    }

    /// Process a message
    ///
    async fn process(&self, data: &interface::core::AmCore, msg: &subscription::Message) {
        println!("process {:?}", msg);

        match msg {
            subscription::Message::ConnectionStatus (status) => {
                println!("ConnectionStatus {:?}", status);
                if status.connected {
                    data.lock().await.events().set_connection_up();
                }
                else {
                    data.lock().await.events().set_connection_down();
                }
            },
            subscription::Message::Mqtt(msg) => {
                
                match msg.get_id() {
                    0 => {
                        data.lock().await.publish_info().await;
                        println!("Ackk !!! {:?}", msg);
                    },
                    _ => {
                        println!("Mqtt {:?}", msg);
                    }
                }

            }
        }

    }

}


struct TestInterfaceStates;

#[async_trait]
impl interface::fsm::States for TestInterfaceStates {

    async fn connecting(&self, core: &AmCore)
    {
        println!("connecting");
        sleep(Duration::from_secs(1)).await;
    }

    async fn initializating(&self, core: &AmCore)
    {
        println!("initializating");
        
        let mut p = core.lock().await;
        p.events().set_init_done();
        sleep(Duration::from_secs(1)).await;
    }

    async fn running(&self, core: &AmCore)
    {
        println!("running");

        sleep(Duration::from_secs(1)).await;
    }

    async fn error(&self, core: &AmCore)
    {
        println!("error");
    }

}



struct TestIdentityProvider {

}

impl interface::IdentityProvider for TestIdentityProvider {

    fn get_info(&self) -> serde_json::Value {
        return serde_json::json!({
            "info": {
                "type": "platform",
                "version": "0.0"
            }
        });
    }

}


struct ServerDeviceActions;


impl DeviceActions for ServerDeviceActions {

    // fn hunt(&self) -> LinkedList<Value> {
    //     return LinkedList::new();
    // }

    /// Create the interfaces
    fn create_interfaces(&self, dev_name: String, bench_name: String, settings: &serde_json::Value)
        -> Vec<AmInterface> {
        let mut list = Vec::new();
        list.push(
            Interface::new(
                "platform", dev_name, bench_name,
                Box::new(TestIdentityProvider{}),
                Box::new(TestInterfaceStates{}),
                Box::new(TestInterfaceListener{})
            )
        );

        return list;
    }
}


pub struct DeviceProducer {

}

impl Producer for DeviceProducer {

    fn create_device(&self) -> Result<Device, String> {
        return Ok(Device::new(Box::new(ServerDeviceActions{})));
    }

}

