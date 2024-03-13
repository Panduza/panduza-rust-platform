use serde_json::{Value};
use std::collections::LinkedList;

use crate::interfaces::Fsm as InterfaceFsm;
use crate::device::{ Device, DeviceActions, Producer };




struct ServerDeviceActions {

}

impl DeviceActions for ServerDeviceActions {

    fn hunt(&self) -> LinkedList<Value> {
        return LinkedList::new();
    }

    fn create_interfaces(&self) -> LinkedList<InterfaceFsm> {
        return LinkedList::new();
    }
}


pub struct DeviceProducer {

}

impl Producer for DeviceProducer {

    fn create_device(&self) -> Result<Device, String> {
        return Ok(Device::new(Box::new(ServerDeviceActions{})));
    }

}

