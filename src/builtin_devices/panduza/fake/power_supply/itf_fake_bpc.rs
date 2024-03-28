use async_trait::async_trait;
use tokio::time::{sleep, Duration};

use crate::subscription;
use crate::interface::{self, Interface};
use crate::interface::core::AmCore;
use crate::interface::AmInterface;
use crate::device::{ Device, traits::DeviceActions, traits::Producer };



struct AttEnable {
    value: bool
}

struct AttVoltage {
    value: f32,
    min: f32,
    max: f32,
    decimals: u16
}

struct AttCurrent {
    value: f32,
    min: f32,
    max: f32,
    decimals: u16
}

struct AttsBpc {
    enable: AttEnable,
    voltage: AttVoltage,
    current: AttCurrent
}




struct ItfFakeBpcSubscriber;




struct ItfIdentityProvider {

}

impl interface::IdentityProvider for ItfIdentityProvider {

    fn get_info(&self) -> serde_json::Value {
        return serde_json::json!({
            "info": {
                "type": "bpc",
                "version": "0.0"
            }
        });
    }

}



struct ItfFakeBpcStates;

#[async_trait]
impl interface::fsm::States for ItfFakeBpcStates {

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




#[async_trait]
impl interface::subscriber::Subscriber for ItfFakeBpcSubscriber {

    async fn attributes_names(&self) -> Vec<(subscription::Id, String)> {
        return vec![
            (0, "enable".to_string()),
            (1, "voltage".to_string()),
            (2, "current".to_string())
        ];
    }

    /// Process a message
    ///
    async fn process(&self, data: &interface::core::AmCore, msg: &subscription::Message) {

        
        match msg {
            subscription::Message::ConnectionStatus (status) => {
                
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




/// Interface to emulate a Bench Power Channel
/// 
pub fn new() -> AmInterface {
    return Interface::new(
        "channel", 
        Box::new(ItfIdentityProvider{}),
        Box::new(ItfFakeBpcStates{}),
        Box::new(ItfFakeBpcSubscriber{})
    );
}
