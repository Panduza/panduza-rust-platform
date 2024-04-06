use std::sync::Arc;

use async_trait::async_trait;

use crate::{interface::{self, core::AmCore, AmInterface}, subscription};
use crate::interface::Builder as InterfaceBuilder;


struct PlatformInterfaceSubscriber;

#[async_trait]
impl interface::subscriber::Subscriber for PlatformInterfaceSubscriber {

    async fn attributes_names(&self) -> Vec<(subscription::Id, String)> {
        return vec![
            (0, "dtree".to_string()),
            (1, "devices".to_string())
        ];
    }

    /// Process a message
    ///
    async fn process(&self, data: &interface::core::AmCore, msg: &subscription::Message) {
        // Common processing
        interface::basic::process(data,msg).await;
        
        match msg {
            subscription::Message::Mqtt(msg) => {
                match msg.id() {
                    
                    _ => {
                        println!("Mqtt {:?}", msg);
                    }
                }

            }
            _ => {}
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
        interface::basic::interface_initializating(core).await;
        
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





///
/// 
pub fn new<A: Into<String>>(name: A) -> InterfaceBuilder {
    return InterfaceBuilder::new(
        name,
        "platform",
        "0.0",
        Box::new(TestInterfaceStates{}),
        Box::new(PlatformInterfaceSubscriber{})
    );
}
