
use async_trait::async_trait;
use serde_json::Value;
use tokio::time::Duration;
use tokio::time::sleep;
use std::clone;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::attribute;


use futures::FutureExt;

use crate::Error as PlatformError;

use crate::interface::ThreadSafeInterface;

use crate::interface::fsm::States as InterfaceStates;

use super::interface;
use super::interface::MetaInterface;

use crate::interface::basic::wait_for_fsm_event;

/// Interfaces are based on a finite state machine
/// Here is the implementation of the states for this meta interface
pub struct MetaStates {
    pub meta_interface: Arc<Mutex<MetaInterface>>
}

#[async_trait]
impl InterfaceStates for MetaStates {

    // ------------------------------------------------------------------------
    /// Just wait for the connection fsm event
    ///
    async fn connecting(&self, interface: &ThreadSafeInterface)
    {
        wait_for_fsm_event(interface).await;
    }

    // ------------------------------------------------------------------------
    /// Initialize the interface
    ///
    async fn initializating(&self, interface: &ThreadSafeInterface)
    -> Result<(), PlatformError>
    {
        let mut meta_interface_locked = self.meta_interface.lock().await;

        

        // Custom initialization slot
        meta_interface_locked.actions.initializating(&interface).await?;


   
        {
            interface.lock().await.add_attribute(
                meta_interface_locked.attribute_settings.clone()
            ).await;

            let mut settings_obj = meta_interface_locked.attribute_settings.lock().await;
            match &mut *settings_obj {
                attribute::Attribute::A1(a) => {
                    a.update_field("value", 0 );
                    a.update_field("timestamp_ms", 0 );
                }
                _ => {}
            }
        }
        
        // Publish all attributes for start
        interface.lock().await.publish_all_attributes().await;


        // let interface_clone = interface.clone();
        // let meta_interface_clone = self.meta_interface.clone();
        // let cos = meta_interface_locked.cyclic_operations.clone();
        // let mut loader = interface.lock().await.platform_services.lock().await.task_loader.clone();
        // loader.load( async move {
        //     loop {
        //         println!("$$$$$ Cyclic operation loop ");

        //         let mut next_awake = 1000;

        //         for co in cos.lock().await.iter() {
        //             println!("$$$ Cyclic operation {:?}", co.interval);

        //             let payload = co.payload.clone();
        //             let oo = serde_json::from_slice::<Value>(&payload).unwrap();
        //             let o = oo.as_object().unwrap();

        //             let index = o.get("index").unwrap().as_u64().unwrap() as usize;
        //             let size = o.get("size").unwrap().as_u64().unwrap() as usize;

        //             // read data back
        //             let r_values = meta_interface_clone.lock().await.actions.read(&interface_clone, index, size).await.unwrap();
        //             println!("r_vals !!! {:?}", r_values);
                    
        //             // update the attribute
        //             {
        //                 let mut meta_interface_locked = meta_interface_clone.lock().await;
                    
        //                 meta_interface_locked.values.splice(index..index+size, r_values.iter().cloned());

                    
        //                 let mut att_map = meta_interface_locked.attribute_map.lock().await;
        //                 match &mut *att_map {
        //                     attribute::Attribute::A3(a) => {
        //                         a.set_payload( meta_interface_locked.to_payload() );
        //                     }
        //                     _ => {}
        //                 }
        //             }

        //             interface_clone.lock().await.publish_all_attributes().await;


        //             if next_awake > co.interval {
        //                 next_awake = co.interval;
        //             }
        //         }

        //         println!("$$$$$ Sleep ");

        //         sleep(Duration::from_millis(next_awake)).await;
    
        //     }
        //     // Ok(())
        // }.boxed()).unwrap();


        // Notify the end of the initialization
        interface.lock().await.set_event_init_done();

        Ok(())
    }

    async fn running(&self, interface: &ThreadSafeInterface)
    {
        println!("running");
        wait_for_fsm_event(interface).await;
    }

    async fn warning(&self, interface: &ThreadSafeInterface)
    {
        // Wait for 5 sec and reboot
        sleep(Duration::from_secs(5)).await;

        interface.lock().await.set_event_reboot();

        println!("warning");
    }

    async fn cleaning(&self, interface: &ThreadSafeInterface)
    {
        println!("cleaning");
        interface.lock().await.set_event_cleaned();
    }
}
