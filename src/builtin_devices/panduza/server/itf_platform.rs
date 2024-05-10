
use async_trait::async_trait;
use serde_json::Value;

use crate::{interface::{self, AmInterface}, subscription};
use crate::interface::Builder as InterfaceBuilder;


struct PlatformInterfaceSubscriber;


impl PlatformInterfaceSubscriber {

    /// 
    /// 
    #[inline(always)]
    async fn process_devices_hunting(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, field_data: &Value) {
        let requested_value = field_data.as_bool().unwrap();
        // self.bpc_interface.lock().await
        //     .actions.write_enable_value(&interface, requested_value).await;

        let platform_services = interface.lock().await
            .platform_services();

        platform_services.lock().await.start_hunting();

        // let r_value = self.bpc_interface.lock().await
        //     .actions.read_enable_value(&interface).await
        //     .unwrap();

        // interface.lock().await
        //     .update_attribute_with_bool("enable", "value", r_value);
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
    async fn process(&self, interface: &AmInterface, msg: &subscription::Message) {
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
                        //         else if attribute_name == "voltage" && field_name == "value" {
                        //             self.process_voltage_value(interface, attribute_name, field_name, field_data).await;
                        //         }
                        //         else if attribute_name == "current" && field_name == "value" {
                        //             self.process_current_value(interface, attribute_name, field_name, field_data).await;
                        //         }
                            }
                        }
                        // interface.lock().await.publish_all_attributes().await;
                    },
                    _ => { }
                }

            }
            _ => {}
        }

    }

}


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
        
        let mut p = interface.lock().await;
        p.set_event_init_done();
    }

    async fn running(&self, interface: &AmInterface)
    {
        let fsm_events_notifier = interface.lock().await.get_fsm_events_notifier();
        fsm_events_notifier.notified().await;
    }

    async fn error(&self, _interface: &AmInterface)
    {
        println!("error");
    }

    async fn cleaning(&self, _interface: &AmInterface)
    {
        println!("cleaning");
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

