use std::{collections::HashMap, sync::Arc};
use nusb::transfer::RequestBuffer;
use nusb::Interface;
// use rumqttc::tokio_rustls::rustls::internal::msgs::message::Message;
use tokio::time::{sleep, Duration};

use tokio;
use lazy_static::lazy_static;
use usbtmc_message::Message;
// use usbtmc_message::BulkInMessage;
use futures_lite::future::block_on;

lazy_static! {
    static ref GATE : tokio::sync::Mutex<Gate>
        = tokio::sync::Mutex::new(Gate { instances: HashMap::new() });
}

pub async fn get(config: &Config) -> Option<UsbtmcConnector> {
    let mut gate = GATE.lock().await;
    gate.get(config)
}


#[derive(Clone, Debug)]
pub struct Config {
    pub usb_vendor: Option<u16>,
    pub usb_model: Option<u16>,
    pub usb_serial: Option<String>,

    pub serial_baudrate: Option<u32>
}

impl Config {
    pub fn new() -> Config {
        Config {
            usb_vendor: None,
            usb_model: None,
            usb_serial: None,
            serial_baudrate: None,
        }
    }

    pub fn import_from_json_settings(&mut self, settings: &serde_json::Value) {

        self.usb_vendor =
            settings.get("usb_vendor")
                .map(|v| v.as_str().unwrap().to_string().parse::<u16>().unwrap());

        self.usb_model =
            settings.get("usb_model")
                .map(|v| v.as_str().unwrap().to_string().parse::<u16>().unwrap());

        self.usb_serial =
            settings.get("usb_serial")
                .map(|v| v.as_str().unwrap().to_string());

    }
}



struct Gate {
    instances: HashMap<String, UsbtmcConnector>
}

impl Gate {


    fn get(&mut self, config: &Config) -> Option<UsbtmcConnector> {
        // First try to get the key
        let key_string = Gate::generate_unique_key_from_config(config)?;
        let key= key_string.as_str();

        // if the instance is not found, it means that the port is not opened yet
        if ! self.instances.contains_key(key) {

            // Create a new instance
            let new_instance = UsbtmcConnector::new(Some(config.clone()));

            // Save the instance
            self.instances.insert(key.to_string(), new_instance.clone());
            tracing::info!(class="Platform", "connector created");
        }

        // Try to find the instance
        let instance = self.instances.get(key)?;

        // Return the instance
        Some(instance.clone())
    }

    /// Try to generate a unique key from the config
    /// This key will be used to find back the tty connector
    ///
    fn generate_unique_key_from_config(config: &Config) -> Option<String> {
        // Check if the usb vendor and model are provided
        if let Some(k) = Some(format!("{}_{}_{}", config.usb_vendor.unwrap(), config.usb_model.unwrap(), config.usb_serial.as_ref().unwrap())) {
            return Some(k.clone());
        }

        // Finally unable to generate a key with the config
        return None;
    }

}



#[derive(Clone)]
pub struct UsbtmcConnector {
    core: Option<Arc<tokio::sync::Mutex<UsbtmcCore>>>,
}

impl UsbtmcConnector {
    
    pub fn new(config: Option<Config>) -> UsbtmcConnector {
        match config {
            Some(config)    => {
                UsbtmcConnector {
                    core: Some(
                        Arc::new(tokio::sync::Mutex::new(
                            UsbtmcCore::new(config)))
                    )
                }
            }
            None            => {
                UsbtmcConnector {
                    core: None
                }
            }
        }
    }

    pub async fn init(&mut self) {
        self.core
            .as_ref()
            .unwrap()
            .lock()
            .await
            .init()
            .await;
    }

    pub async fn write(&mut self, command: &[u8],
        time_lock: Option<Duration>) 
            -> Result<usize> {
        self.core
            .as_ref()
            .unwrap()
            .lock()
            .await
            .write(command, time_lock)
            .await
    }


    pub async fn write_then_read(&mut self, command: &[u8], response: &mut [u8],
        time_lock: Option<Duration>) 
            -> Result<usize> {
        self.core
            .as_ref()
            .unwrap()
            .lock()
            .await
            .write_then_read(command, response, time_lock)
            .await
    }

}




struct TimeLock {
    duration: tokio::time::Duration,
    t0: tokio::time::Instant
}


struct UsbtmcCore {
    config: Config,
    // builder: Option< SerialPortBuilder >,
    // serial_stream: Option< SerialStream >,
    time_lock: Option<TimeLock>
}

impl UsbtmcCore {

    fn new(config: Config) -> UsbtmcCore {
        UsbtmcCore {
            config: config,
            // builder: None,
            // serial_stream: None,
            time_lock: None
        }
    }

    async fn init(&mut self) {

        let devices = nusb::list_devices()
            .unwrap()
            .find(|d| d.vendor_id() == self.config.usb_vendor.unwrap() && d.product_id() == self.config.usb_model.unwrap())
            .expect("device is not connected");

        println!("Device info: {devices:?}");

        // let device = devices.open().unwrap();
        // let interface = device.claim_interface(0).unwrap();
        // interface.clear_halt(0x02).unwrap();


        // // dirty fix, need to be improved
        // if self.serial_stream.is_some() {
        //     return;
        // }

        // let serial_port_name = Some(format!("{}_{}_{}", self.config.usb_vendor.unwrap(), self.config.usb_model.unwrap(), self.config.usb_serial.as_ref().unwrap()));

        // let serial_builder = tokio_serial::new(
        //     serial_port_name.as_ref().unwrap(),
        //     self.config.serial_baudrate.unwrap()

        // );        

        // let pp = SerialStream::open(&serial_builder);
        // let aa = pp.expect("pok");

        
        // self.builder = Some(serial_builder);
        // self.serial_stream = Some(aa);

    }


    async fn time_locked_write(&mut self, interface: Interface, sequence: Vec<Message>, endpoint_out: u8, duration: Option<Duration>)-> Result<usize> {


        if let Some(lock) = self.time_lock.as_mut() {
            let elapsed = tokio::time::Instant::now() - lock.t0;
            if elapsed < lock.duration {
                let wait_time = lock.duration - elapsed;
                sleep(wait_time).await;
            }
            self.time_lock = None;
        }

        // Send the sequence on the usb
        for i in 0..sequence.len() {
            let message = sequence[i].to_vec();
            // SEND TO USB
            let rrr = block_on(interface.bulk_out(endpoint_out, message.to_vec()))
                .into_result()
                .unwrap();
            
            println!("{:?}", rrr);
        }
        // let rrr = self.serial_stream.as_mut().unwrap().write(command).await;

        // Set the time lock
        if let Some(duration) = duration {
            self.time_lock = Some(TimeLock {
                duration: duration,
                t0: tokio::time::Instant::now()
            });
        }

        // rrr
    }

    
    async fn write(&mut self, interface: Interface, sequence: Vec<Message>, endpoint_out: u8,
        time_lock: Option<Duration>) 
            -> Result<usize> {

        self.time_locked_write(interface, sequence, endpoint_out, time_lock).await
    }

    async fn write_then_read(&mut self, interface: Interface, sequence: Vec<Message>, endpoint_out: u8, endpoint_in: u8, response: RequestBuffer,
        time_lock: Option<Duration>) 
            -> String{


        let _ = self.time_locked_write(interface, sequence, endpoint_out, time_lock).await;


        // let mut buf: &mut [u8] = &mut [0; 1024];
        // self.serial_stream.as_mut().unwrap().read(response).await
        
        // let resp_buf = nusb::transfer::RequestBuffer::new(64 as usize);
        let data = block_on(interface.bulk_in(endpoint_in, response)).into_result().unwrap();
        println!("{:?}", data);

        // Parse the received data
        let msg = usbtmc_message::BulkInMessage::from_u8_array(&data);

        msg.payload_as_string()

        // let n = p.unwrap();
        // println!("Read {} bytes", n);

        

    }

    async fn read(&mut self, interface: Interface, endpoint_in: u8, response: RequestBuffer) {
        // let resp_buf = nusb::transfer::RequestBuffer::new(64 as usize);
        let data = block_on(interface.bulk_in(endpoint_in, response)).into_result().unwrap();
        println!("{:?}", data);

        // Parse the received data
        let msg = usbtmc_message::BulkInMessage::from_u8_array(&data);

        msg.payload_as_string();
    }


}