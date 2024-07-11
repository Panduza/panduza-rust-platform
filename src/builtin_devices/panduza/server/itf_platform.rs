
use async_trait::async_trait;
use serde_json::{json, Value};

use panduza_core::{attribute::JsonAttribute, interface::{self, AmInterface}, subscription};
use panduza_core::interface::Builder as InterfaceBuilder;

use panduza_core::FunctionResult as PlatformFunctionResult;
use panduza_core::Error as PlatformError;

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

struct PlatformInterfaceSubscriber;


impl PlatformInterfaceSubscriber {

    /// 
    /// 
    #[inline(always)]
    async fn process_devices_hunting(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, field_data: &Value) {
        let requested_value = field_data.as_bool().unwrap();
        // self.bpc_interface.lock().await
        //     .actions.write_enable_value(&interface, requested_value).await;

        if requested_value == true {

            // Hunting is going to start
            let _ = interface.lock().await.update_attribute_with_bool("devices", "hunting", true);

            interface.lock().await
                .publish_all_attributes().await;

            let platform_services = interface.lock().await
                .platform_services();

            platform_services.lock().await.start_hunting();

            while platform_services.lock().await.is_hunt_in_progress() {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }

            interface.lock().await
                .update_attribute_with_json("devices", "store", 
                    platform_services.lock().await.get_device_store()
                );

            // Hunting has finished
            let _ = interface.lock().await.update_attribute_with_bool("devices", "hunting", false);
            interface.lock().await
                .publish_all_attributes().await;
        }

    }

    /// 
    /// 
    #[inline(always)]
    async fn process_dtree_content(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, field_data: &Value) {
        let obj = field_data.as_object().unwrap();

        println!("process_dtree_content: {:?}", obj);

        let platform_services = 
            interface.lock().await.platform_services();

        platform_services.lock().await
            .set_tree_content(
                serde_json::Value::Object(obj.clone()) );
    }


}


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
    
                        println!("PZA_CMDS_SET: {:?}", o);
    
                        for (attribute_name, fields) in o.iter() {
                            for (field_name, field_data) in fields.as_object().unwrap().iter() {
                                if attribute_name == "devices" && field_name == "hunting" {
                                    self.process_devices_hunting(&interface, attribute_name, field_name, field_data).await;
                                }
                                else if attribute_name == "dtree" && field_name == "content" {
                                    self.process_dtree_content(interface, attribute_name, field_name, field_data).await;
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

    async fn initializating(&self, interface: &AmInterface) -> Result<(), PlatformError>
    {
        interface::basic::interface_initializating(interface).await;
        
        interface.lock().await.register_attribute(JsonAttribute::new_boxed("dtree", true));
        interface.lock().await.register_attribute(JsonAttribute::new_boxed("devices", true));


        let mut ii = interface.lock().await;
        let ps = ii.platform_services().clone();

        let store = ps.lock().await.get_device_store().clone();

        ii.update_attribute_with_json("devices", "store", 
            &store
        );

        match store.as_object() {
            Some(store_map) => {
                ii.update_attribute_with_json("devices", "max", &serde_json::Value::Number(
                    store_map.len().into()
                ));
            },  
            None => {
                // If max not found don't publish max
            }
        }

        ii.update_attribute_with_string("dtree", "name", &"pok".to_string());
        ii.update_attribute_with_json("dtree", "content", &json!({ "a": 1 }));

        ii.publish_all_attributes().await;


        ii.set_event_init_done();

        Ok(())
    }

    async fn running(&self, interface: &AmInterface)
    {
        let fsm_events_notifier = interface.lock().await.get_fsm_events_notifier();
        fsm_events_notifier.notified().await;
    }

    async fn warning(&self, _interface: &AmInterface)
    {
        println!("error");
    }

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
        "platform",
        "0.0",
        Box::new(TestInterfaceStates{}),
        Box::new(PlatformInterfaceSubscriber{})
    );
}

