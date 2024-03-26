use async_trait::async_trait;

use crate::{interface::{self, core::AmCore, AmInterface, Interface}, subscription};




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


/// Interface to emulate a Bench Power Channel
/// 
pub fn new<A: Into<String>>(name: A) -> AmInterface {
    return Interface::new(
        name,
        Box::new(TestIdentityProvider{}),
        Box::new(TestInterfaceStates{}),
        Box::new(PlatformInterfaceSubscriber{})
    );
}


