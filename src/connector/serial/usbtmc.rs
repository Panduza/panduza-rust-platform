// use std::{collections::HashMap, sync::Arc};

// use tokio;
// use lazy_static::lazy_static;

// lazy_static! {
//     static ref GATE : tokio::sync::Mutex<Gate>
//         = tokio::sync::Mutex::new(Gate { instances: HashMap::new() });
// }

// pub async fn get(config: &Config) -> Option<UsbtmcConnector> {
//     let mut gate = GATE.lock().await;
//     gate.get(config)
// }


// #[derive(Clone, Debug)]
// pub struct Config {
//     pub usb_vendor: Option<u16>,
//     pub usb_model: Option<u16>,
//     pub usb_serial: Option<String>,
// }

// impl Config {
//     pub fn new() -> Config {
//         Config {
//             usb_vendor: None,
//             usb_model: None,
//             usb_serial: None,
//         }
//     }

//     pub fn import_from_json_settings(&mut self, settings: &serde_json::Value) {

//         self.usb_vendor =
//             settings.get("usb_vendor")
//                 .map(|v| v.as_str().unwrap().to_string().parse::<u16>().unwrap());

//         self.usb_model =
//             settings.get("usb_model")
//                 .map(|v| v.as_str().unwrap().to_string().parse::<u16>().unwrap());

//         self.usb_serial =
//             settings.get("usb_serial")
//                 .map(|v| v.as_str().unwrap().to_string());

//     }
// }



// struct Gate {
//     instances: HashMap<String, UsbtmcConnector>
// }

// impl Gate {


//     fn get(&mut self, config: &Config) -> Option<UsbtmcConnector> {
//         // First try to get the key
//         let key_string = Gate::generate_unique_key_from_config(config)?;
//         let key= key_string.as_str();

//         // if the instance is not found, it means that the port is not opened yet
//         if ! self.instances.contains_key(key) {

//             // Create a new instance
//             let new_instance = UsbtmcConnector::new(Some(config.clone()));

//             // Save the instance
//             self.instances.insert(key.to_string(), new_instance.clone());
//             tracing::info!(class="Platform", "connector created");
//         }

//         // Try to find the instance
//         let instance = self.instances.get(key)?;

//         // Return the instance
//         Some(instance.clone())
//     }

//     /// Try to generate a unique key from the config
//     /// This key will be used to find back the tty connector
//     ///
//     fn generate_unique_key_from_config(config: &Config) -> Option<String> {
//         // Check if the usb vendor and model are provided
//         if let Some(k) = Some(format!("{}_{}", config.usb_vendor.unwrap(), config.usb_model.unwrap())) {
//             return Some(k.clone());
//         }

//         // Finally unable to generate a key with the config
//         return None;
//     }

// }



// #[derive(Clone)]
// pub struct UsbtmcConnector {
//     core: Option<Arc<tokio::sync::Mutex<UsbtmcCore>>>,
// }

// impl UsbtmcConnector {
    
//     pub fn new(config: Option<Config>) -> UsbtmcConnector {
//         match config {
//             Some(config)    => {
//                 UsbtmcConnector {
//                     core: Some(
//                         Arc::new(tokio::sync::Mutex::new(
//                             UsbtmcCore::new(config)))
//                     )
//                 }
//             }
//             None            => {
//                 UsbtmcConnector {
//                     core: None
//                 }
//             }
//         }
//     }

//     pub async fn init(&mut self) {
//         self.core
//             .as_ref()
//             .unwrap()
//             .lock()
//             .await
//             .init()
//             .await;
//     }

//     pub async fn write(&mut self, command: &[u8],
//         time_lock: Option<Duration>) 
//             -> Result<usize> {
//         self.core
//             .as_ref()
//             .unwrap()
//             .lock()
//             .await
//             .write(command, time_lock)
//             .await
//     }


//     pub async fn write_then_read(&mut self, command: &[u8], response: &mut [u8],
//         time_lock: Option<Duration>) 
//             -> Result<usize> {
//         self.core
//             .as_ref()
//             .unwrap()
//             .lock()
//             .await
//             .write_then_read(command, response, time_lock)
//             .await
//     }

// }




// struct TimeLock {
//     duration: tokio::time::Duration,
//     t0: tokio::time::Instant
// }


