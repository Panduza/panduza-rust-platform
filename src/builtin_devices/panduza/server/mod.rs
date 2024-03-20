

use crate::interface;
use crate::interface::core::AmCore;
use crate::interface::AmInterface;
use crate::device::{ Device, DeviceActions, Producer };

use async_trait::async_trait;

use tokio::time::{sleep, Duration};

// use crate::connection::LinkInterfaceHandle;

use crate::subscription;


struct TestInterfaceListener;

#[async_trait]
impl interface::listener::Subscriber for TestInterfaceListener {


    // fn get_info(&self) -> Value {
    //     return json!({
    //         "info": {
    //             "type": "platform",
    //             "version": "0.0"
    //         }
    //     })
    // }


    /// List of subscription requests
    /// 
    async fn subscription_requests(&self) -> Vec<subscription::Request> {
        return vec![
            subscription::Request::new( 0, "pza" )
        ];
    }

    /// Process a message
    /// 
    async fn process(&self, data: &interface::core::AmCore, msg: &subscription::Message) {
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
                
                match msg.get_id() {
                    0 => {
                        data.lock().await.publish_info().await;
                        println!("Ackk !!! {:?}", msg);
                    },
                    _ => {
                        println!("Mqtt {:?}", msg);
                    }
                }

            }
        }

    }

}


struct TestInterfaceStates {

}

#[async_trait]
impl interface::fsm::States for TestInterfaceStates {


    async fn connecting(&self, data: &AmCore)
    {
        println!("connecting");
        sleep(Duration::from_secs(1)).await;
    }
    async fn initializating(&self, data: &AmCore)
    {
        println!("initializating");
        
        data.lock().await.events.set_init_done();
        sleep(Duration::from_secs(1)).await;
    }
    async fn running(&self, data: &AmCore)
    {
        println!("running");

        sleep(Duration::from_secs(1)).await;
    }
    async fn error(&self, data: &AmCore)
    {
        println!("error");
    }

}




struct ServerDeviceActions {

}

impl DeviceActions for ServerDeviceActions {

    // fn hunt(&self) -> LinkedList<Value> {
    //     return LinkedList::new();
    // }

    fn create_interfaces<A: Into<String>, B: Into<String>>
        (&self, dev_name: A, bench_name: B, settings: &serde_json::Value) -> Vec<AmInterface> {
        let mut list = Vec::new();
        // list.push_back(
        //     Arc::new(Mutex::new(
        //         Interface::new(
        //             "platform",
        //             Box::new(TestInterfaceStates{}),
        //             Box::new(TestInterfaceListener{})
        //     )
        //     ))
        // );

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

