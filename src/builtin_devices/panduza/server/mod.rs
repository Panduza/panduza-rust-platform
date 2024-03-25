use async_trait::async_trait;
use tokio::time::{sleep, Duration};

use crate::subscription;
use crate::interface::{self, Interface};
use crate::interface::core::AmCore;
use crate::interface::AmInterface;
use crate::device::{ Device, DeviceActions, Producer };

struct PlatformInterfaceSubscriber;

#[async_trait]
impl interface::listener::Subscriber for PlatformInterfaceSubscriber {

    async fn attributes_names(&self) -> Vec<(subscription::Id, String)> {
        return vec![
            (0, "dtree".to_string()),
            (1, "devices".to_string())
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
                    data.lock().await.set_event_connection_up();
                }
                else {
                    data.lock().await.set_event_connection_down();
                }
            },
            subscription::Message::Mqtt(msg) => {
                
                match msg.get_id() {
                    subscription::ID_PZA => {
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

        let fsm_events_notifier = core.lock().await.get_fsm_events_notifier();
        fsm_events_notifier.notified().await;
    }

    async fn initializating(&self, core: &AmCore)
    {
        println!("initializating");
        
        let mut p = core.lock().await;
        p.set_event_init_done();
    }

    async fn running(&self, core: &AmCore)
    {
        println!("running");
        
        let fsm_events_notifier = core.lock().await.get_fsm_events_notifier();
        fsm_events_notifier.notified().await;
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
                Box::new(PlatformInterfaceSubscriber{})
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

