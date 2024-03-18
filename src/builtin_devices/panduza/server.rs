use rumqttc::tokio_rustls::rustls::internal::msgs;
use serde_json::Value;
use tokio::sync::Mutex;
use std::collections::LinkedList;
use std::sync::Arc;


use crate::interface::{Event, SafeInterface, StateImplementations, HandlerImplementations};
use crate::interface::Interface;
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

    async fn process(&self, msg: &subscription::Message) {
        println!("process {:?}", msg);

        match msg {
            subscription::Message::ConnectionStatus (status) => {
                println!("ConnectionStatus {:?}", status);
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




    async fn poll_events(&self) -> Vec<Event> {
        return vec![Event::NoEvent];
    }

    async fn enter_connecting(&self) {
        println!("enter_connecting ");

        // for link in _links.iter() {
        //     link.topic_subscriber_tx.send("hello!!!!!!!!!!!!!!!!!!!!".to_string()).await.unwrap();
        // }

        // .unwrap().send("hello".to_string()).await;

        sleep(Duration::from_secs(1)).await;
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

