
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{attribute, subscription};


use crate::interface::ThreadSafeInterface;

use crate::interface::fsm::States as InterfaceStates;

use super::interface::MetaInterface;

use crate::interface::basic::wait_for_fsm_event;

// Interface is based on a finite state machine
// Here is the implementation of the states for the registers interface

pub struct MetaStates {
    pub meta_interface: Arc<Mutex<MetaInterface>>
}

#[async_trait]
impl InterfaceStates for MetaStates {




    /// Just wait for an fsm event for the connection
    ///
    async fn connecting(&self, interface: &ThreadSafeInterface)
    {
        wait_for_fsm_event(interface).await;
    }

    /// Initialize the interface
    ///
    async fn initializating(&self, interface: &ThreadSafeInterface)
    {
        let mut registers_itf = self.meta_interface.lock().await;

        // Custom initialization slot
        registers_itf.actions.initializating(&interface).await.unwrap();


        {        
            let map =
                interface.lock().await.create_attribute(
                    attribute::Attribute::A3(attribute::A3::new("map"))
                );
            
            let mut map_obj = map.lock().await;
            match &mut *map_obj {
                attribute::Attribute::A3(a) => {
                    a.set_payload(vec![1, 2, 3, 4, 5, 6, 7, 8]);
                }
                _ => {}
            }
        }
            

        {        
            let settings =
                interface.lock().await.create_attribute(
                    attribute::Attribute::A1(attribute::A1::new("settings"))
                );
        
            let mut settings_obj = settings.lock().await;
            match &mut *settings_obj {
                attribute::Attribute::A1(a) => {
                    a.update_field("base_address", 0);
                    a.update_field("register_size", 0);
                    a.update_field("number_of_register", 0);
                }
                _ => {}
            }
        }
        
        // Publish all attributes for start
        interface.lock().await.publish_all_attributes().await;

        // Notify the end of the initialization
        interface.lock().await.set_event_init_done();
    }

    async fn running(&self, interface: &ThreadSafeInterface)
    {
        println!("running");


        wait_for_fsm_event(interface).await;
    }

    async fn error(&self, _interface: &ThreadSafeInterface)
    {
        println!("error");
    }

    // async fn cleaning(&self, _interface: &ThreadSafeInterface)
    // {
    //     println!("cleaning");
    // }
}
