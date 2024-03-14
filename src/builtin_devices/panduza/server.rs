use serde_json::{Value};
use std::collections::LinkedList;


use crate::interfaces::{Event, StateImplementations};
use crate::interfaces::Fsm as InterfaceFsm;
use crate::device::{ Device, DeviceActions, Producer };

use async_trait::async_trait;





struct TestInterface {

}


#[async_trait]
impl StateImplementations for TestInterface {
    
        async fn poll_events(&self) -> Vec<Event> {
            return vec![Event::NoEvent];
        }
    
        async fn enter_connecting(&self) {
            // println!("enter_connecting");
        }
    
        async fn state_connecting(&self) {
            println!("state_connecting");
        }
    
        async fn leave_connecting(&self) {
            println!("leave_connecting");
        }
    
        async fn enter_running(&self) {
            println!("enter_running");
        }
    
        async fn state_running(&self) {
            println!("state_running");
        }
    
        async fn leave_running(&self) {
            println!("leave_running");
        }
    
}




struct ServerDeviceActions {

}

impl DeviceActions for ServerDeviceActions {

    fn hunt(&self) -> LinkedList<Value> {
        return LinkedList::new();
    }

    fn create_interfaces(&self) -> LinkedList<InterfaceFsm> {
        let mut list = LinkedList::new();
        list.push_back(
            InterfaceFsm::new(Box::new(TestInterface{}))
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
