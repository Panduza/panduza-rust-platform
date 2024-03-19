use rumqttc::tokio_rustls::rustls::internal::msgs;
use serde_json::Value;
use tokio::sync::Mutex;
use std::collections::LinkedList;
use std::sync::Arc;


use crate::interface::{ SafeInterface, StateImplementations, HandlerImplementations};
use crate::interface::Interface;
use crate::interface;
use crate::device::{ Device, DeviceActions, Producer };

use async_trait::async_trait;

use tokio::time::{sleep, Duration};

// use crate::connection::LinkInterfaceHandle;

use crate::subscription::Request as SubscriptionRequest;
use crate::subscription;



// 

struct TestInterfaceListener {

}

#[async_trait]
impl HandlerImplementations for TestInterfaceListener {

    async fn get_subscription_requests(&self) -> Vec<SubscriptionRequest> {
        return vec![
            SubscriptionRequest::new( 0, "pza" )
        ];
    }

    async fn process(&self, data: &interface::SafeData, msg: &subscription::Message) {
        println!("process {:?}", msg);

        match msg {
            subscription::Message::ConnectionStatus (status) => {
                println!("ConnectionStatus {:?}", status);
                if status.connected {
                    data.lock().await.events.set_connection_up();
                }
                else {
                    data.lock().await.events.set_connection_down();
                }
            },
            subscription::Message::Mqtt(msg) => {
                
            }
        }

    }

}


struct TestInterfaceStates {

}

#[async_trait]
impl StateImplementations for TestInterfaceStates {


    async fn connecting(&self)
    {
        println!("connecting");
    }
    async fn initializating(&self)
    {
        println!("initializating");
    }
    async fn running(&self)
    {
        println!("running");
    }
    async fn error(&self)
    {
        println!("error");
    }

}




struct ServerDeviceActions {

}

impl DeviceActions for ServerDeviceActions {

    fn hunt(&self) -> LinkedList<Value> {
        return LinkedList::new();
    }

    fn create_interfaces(&self) -> LinkedList<SafeInterface> {
        let mut list = LinkedList::new();
        list.push_back(
            Arc::new(Mutex::new(
                Interface::new(Box::new(TestInterfaceStates{}),
                    Box::new(TestInterfaceListener{})      
            )
            ))
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

