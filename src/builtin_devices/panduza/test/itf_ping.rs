
// use std::str::FromStr;

use async_trait::async_trait;
use serde_json::Value;

use panduza_core::{attribute::JsonAttribute, interface::{self, AmInterface}, subscription};
use panduza_core::interface::Builder as InterfaceBuilder;

use panduza_core::FunctionResult as PlatformFunctionResult;

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

// static mut DEBUG_COUNTER: u32 = 0;

struct PingInterfaceSubscriber;

impl PingInterfaceSubscriber {

    /// 
    /// 
    #[inline(always)]
    async fn process_devices_hunting(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, field_data: &Value) {

        // unsafe { DEBUG_COUNTER += 1;
        // println!("process_devices_hunting: {:?}", DEBUG_COUNTER);
        // };

        interface.lock().await
            .update_attribute_with_string("mirror", "value", 
                &field_data.as_str().unwrap().to_string()
            );
        interface.lock().await
            .publish_all_attributes().await;
    
    }
}


#[async_trait]
impl interface::subscriber::Subscriber for PingInterfaceSubscriber {

    async fn attributes_names(&self) -> Vec<(subscription::Id, String)> {
        return vec![
        ];
    }

    /// Process a message
    ///
    async fn process(&self, interface: &AmInterface, msg: &subscription::Message) -> PlatformFunctionResult {
        // Common processing
        interface::basic::process(interface,msg).await;
        
        match msg {
            subscription::Message::Mqtt(msg) => {
                match msg.id() {
                    
                    subscription::ID_PZA_CMDS_SET => {
                        let payload = msg.payload();
                        let oo = serde_json::from_slice::<Value>(payload).unwrap();
                        let o = oo.as_object().unwrap();
    
                        // println!("PZA_CMDS_SET: {:?}", o);
    
                        for (attribute_name, fields) in o.iter() {
                            for (field_name, field_data) in fields.as_object().unwrap().iter() {
                                if attribute_name == "mirror" && field_name == "value" {
                                    self.process_devices_hunting(&interface, attribute_name, field_name, field_data).await;
                                }
                            }
                        }
                        // interface.lock().await.publish_all_attributes().await;
                    },
                    _ => { }
                }

            }
            _ => {}
        }

        Ok(())

    }

}


// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

struct TestInterfaceStates;

#[async_trait]
impl interface::fsm::States for TestInterfaceStates {

    async fn connecting(&self, interface: &AmInterface)
    {
        let fsm_events_notifier = interface.lock().await.get_fsm_events_notifier();
        fsm_events_notifier.notified().await;
    }

    async fn initializating(&self, interface: &AmInterface)
    {
        interface::basic::interface_initializating(interface).await;
        
        interface.lock().await.register_attribute(JsonAttribute::new_boxed("mirror", true));


        let mut ii = interface.lock().await;

        ii.update_attribute_with_string("mirror", "value", &"".to_string());

        ii.publish_all_attributes().await;


        ii.set_event_init_done();
    }

    async fn running(&self, interface: &AmInterface)
    {
        let fsm_events_notifier = interface.lock().await.get_fsm_events_notifier();
        fsm_events_notifier.notified().await;
    }

    // async fn warning(&self, _interface: &AmInterface)
    // {
    //     println!("error");
    // }

    async fn cleaning(&self, _interface: &AmInterface)
    {
        println!("cleaning");
    }
}




// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------


///
/// 
pub fn new<A: Into<String>>(name: A) -> InterfaceBuilder {
    return InterfaceBuilder::new(
        name,
        "ping",
        "0.0",
        Box::new(TestInterfaceStates{}),
        Box::new(PingInterfaceSubscriber{})
    );
}

