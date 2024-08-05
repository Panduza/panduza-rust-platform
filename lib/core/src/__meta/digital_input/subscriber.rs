use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{attribute, interface, subscription};

use serde_json::Value;

use crate::interface::ThreadSafeInterface;
use super::interface::MetaInterface;

use crate::interface::subscriber::Subscriber as InterfaceSubscriber;

use crate::FunctionResult as PlatformFunctionResult;

use crate::interface::basic::process as basic_process;

use super::interface::CyclicOperation;

const ID_COMMAND: subscription::Id = 0;
// const ID_ENABLE: subscription::Id = 1;
// const ID_POWER: subscription::Id = 2;
// const ID_CURRENT: subscription::Id = 3;

pub struct MetaSubscriber {
    pub meta_interface: Arc<Mutex<MetaInterface>>
}

impl MetaSubscriber {

    // /// 
    // /// 
    // #[inline(always)]
    // async fn process_mode_value(&self, interface: &ThreadSafeInterface, _attribute_name: &str, _field_name: &str, field_data: &Value) {
    //     let requested_value = field_data.as_str().unwrap().to_string();
    //     self.reg_interface.lock().await
    //         .actions.write_mode_value(&interface, requested_value).await;

    //     let r_value = self.reg_interface.lock().await
    //         .actions.read_mode_value(&interface).await
    //         .unwrap();

    //     interface.lock().await
    //         .update_attribute_with_string("mode", "value", &r_value);
    // }



}

#[async_trait]
impl InterfaceSubscriber for MetaSubscriber {

    /// Get the list of attributes names
    ///
    async fn attributes_names(&self) -> Vec<(subscription::Id, String)> {
        return vec![
            (ID_COMMAND, "command".to_string()),
        ];
    }


    /// Process a message
    ///
    async fn process(&self, generic_interface: &ThreadSafeInterface, msg: &subscription::Message) -> PlatformFunctionResult {
        // Common processing
        basic_process(&generic_interface, msg).await;

        match msg {
            subscription::Message::Mqtt(msg) => {

                println!("!!!!!!!!!!!   MetaSubscriber::process: {:?}", msg.topic());

                match msg.id() {
                // subscription::ID_PZA_CMDS_SET => {
                //     // interface.lock().await.publish_info().await;

                //     // only when running state

                //     println!("MetaSubscriber::process: {:?}", msg.topic());
                //     println!("MetaSubscriber::process: {:?}", msg.payload());

                //     let payload = msg.payload();
                //     let oo = serde_json::from_slice::<Value>(payload).unwrap();
                //     // let o = oo.as_object().unwrap();


                //     // for (attribute_name, fields) in o.iter() {
                //     //     for (field_name, field_data) in fields.as_object().unwrap().iter() {
                //     //         if attribute_name == "mode" && field_name == "value" {
                //     //             self.process_mode_value(&interface, attribute_name, field_name, field_data).await;
                //     //         }
                //     //         else if attribute_name == "enable" && field_name == "value" {
                //     //             self.process_enable_value(&interface, attribute_name, field_name, field_data).await;
                //     //         }
                //     //         else if attribute_name == "power" && field_name == "value" {
                //     //             self.process_power_value(interface, attribute_name, field_name, field_data).await;
                //     //         }
                //     //         else if attribute_name == "current" && field_name == "value" {
                //     //             self.process_current_value(interface, attribute_name, field_name, field_data).await;
                //     //         }
                //     //     }
                //     // }
                //     // interface.lock().await.publish_all_attributes().await;


                // },
                ID_COMMAND => {
                    println!("command !!! {:?}", msg.payload());
                    let payload = msg.payload();
                    let oo = serde_json::from_slice::<Value>(payload).unwrap();
                    let o = oo.as_object().unwrap();
                    println!("command !!! {:?}", o);

                //     if o.get("cmd").unwrap().as_str().unwrap() == "w" {
                //         let index = o.get("index").unwrap().as_u64().unwrap() as usize;
                //         let values = o.get("values").unwrap().as_array().unwrap();

                //         println!("command !!! {:?}", values);

                //         // write data
                //         let values_u64: Vec<u64> = values.iter().map(|v| v.as_u64().unwrap()).collect();
                //         self.meta_interface.lock().await.actions.write(&generic_interface, index, &values_u64).await;

                //         // read data back
                //         let r_values = self.meta_interface.lock().await.actions.read(&generic_interface, index, values_u64.len()).await.unwrap();
                //         println!("r_vals !!! {:?}", r_values);
                        
                //         // update the attribute
                //         {
                //             let mut meta_interface_locked = self.meta_interface.lock().await;
                        
                //             meta_interface_locked.values.splice(index..index+values_u64.len(), r_values.iter().cloned());

                        
                //             let mut att_map = meta_interface_locked.attribute_map.lock().await;
                //             match &mut *att_map {
                //                 attribute::Attribute::A3(a) => {
                //                     a.set_payload( meta_interface_locked.to_payload() );
                //                 }
                //                 _ => {}
                //             }
                //         }
                                        
                //         // Publish all attributes for start
                //         generic_interface.lock().await.publish_all_attributes().await;

                        
                //     }
                //     else if o.get("cmd").unwrap().as_str().unwrap() == "r" {
                //         let index = o.get("index").unwrap().as_u64().unwrap() as usize;
                //         let size = o.get("size").unwrap().as_u64().unwrap() as usize;

                //         let repeat_opt = o.get("repeat");
                //         if repeat_opt.is_some() {
                //             println!("????????????repeat !!! {:?}", repeat_opt.unwrap().as_u64().unwrap());
                //             let repeat = repeat_opt.unwrap().as_u64().unwrap();
                            
                //             self.meta_interface.lock().await.cyclic_operations.lock().await.push_back(
                //                 CyclicOperation {
                //                     interval: repeat,
                //                     payload: payload.clone()
                //                 }
                //             );
                //         }


                //         // read data back
                //         let r_values = self.meta_interface.lock().await.actions.read(&generic_interface, index, size).await.unwrap();
                //         println!("r_vals !!! {:?}", r_values);
                        
                //         // update the attribute
                //         {
                //             let mut meta_interface_locked = self.meta_interface.lock().await;
                        
                //             meta_interface_locked.values.splice(index..index+size, r_values.iter().cloned());

                        
                //             let mut att_map = meta_interface_locked.attribute_map.lock().await;
                //             match &mut *att_map {
                //                 attribute::Attribute::A3(a) => {
                //                     a.set_payload( meta_interface_locked.to_payload() );
                //                 }
                //                 _ => {}
                //             }
                //         }
                                        
                //         // Publish all attributes for start
                //         generic_interface.lock().await.publish_all_attributes().await;
                        
                //     }
                }
                _ => {
                    // not managed by the common level
                }
                }
            }
            _ => {
                // not managed by the common level
            }
        }

        Ok(())
    }


}
