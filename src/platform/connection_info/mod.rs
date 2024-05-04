
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;
use std::fs::File;
use std::io::Read;

use crate::platform::PlatformError;

mod tests;

#[derive(Debug)]
pub struct ConnectionInfo {
    
    // broker info
    hostname: String,
    port: u16,

    // credential
    
}

impl ConnectionInfo {

    /// Create a new ConnectionInfo object with default values
    ///
    fn default() -> Self {
        Self {
            hostname: "localhost".to_string(),
            port: 1883,
        }
    }

    fn from_json_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Read the file contents
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Parse the JSON contents
        let parsed_data: serde_json::Value = serde_json::from_str(&contents)?;

        // Process the parsed data
        // TODO: Implement your logic here


        Ok(())
    }

    fn from_json_string(&self, json_string: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Parse the JSON contents
        // let parsed_data: serde_json::Value = serde_json::from_str(json_string)?;

        
        // Process the parsed data
    
        Ok(())
    }


    /// Create a new ConnectionInfo object from a JSON value
    ///
    fn from_json_value(json_obj: JsonValue) -> Result<Self, &'static str> {
        json_obj.as_object()
                .ok_or("Except a JSON object at file root")
                .and_then(ConnectionInfo::from_map_object)
    }

    ///
    ///
    fn from_map_object(map_obj: &JsonMap<String, JsonValue>) -> Result<Self, &'static str> {
        
        
        let hostname = 
            map_obj.get("broker_host")
            .and_then(|v| v.as_str())
            .ok_or("hostname not provided in network.json, continue with default host")?;
            // .ok_or_else(err)
            // .or("localhost");
            // .ok_or("hostname not provided in network.json, continue with default host");
        
        // Implement the logic here
        
        Err("pokkk")
        // Ok(
        //     Self {
        //         hostname: "localhost".to_string(),
        //         port: 1883,
        //     }
        // )
    }

    /// Extract the hostname from the JSON object
    ///
    fn extract_hostname_from_json_object(&self, obj: JsonValue) -> Result<(), PlatformError> {

        // let hostname_json_value = 
        
        // let hostname_json_value = 
        //     obj.get("broker_host")
        //     .or_else(|| obj.get("host"));


        // hostname_json_value.ok_or(PlatformError::new("host not provided in network.json, continue with default host"))?;

        // obj.get("broker_host")
        // .a
            // .ok_or(|)


        // match hostname_json_value {
        //     Some(host) => {


        //         match host {
        //             JsonValue::String(_) => {
        //                 // ...
        //             }
        //             _ => {
        //                 // ...
        //             }
        //         }

        //         // if let Some(JsonValue::String(host)) = hostname_json_value {
        //         //     tracing::info!(class="Platform", "host: {}", host);
        //         // } else {
        //         //     tracing::warn!(class="Platform", "host not provided in network.json, continue with default host");
        //         //     "localhost"
        //         // }

        //         if host == JsonValue::String {
        //             let host_str = host.as_str().unwrap();
        //             tracing::info!(class="Platform", "host: {}", host_str);
        //         }

        //         // host.as_str().unwrap()
        //     }
        //     None => {
        //         tracing::warn!(class="Platform", "host not provided in network.json, continue with default host");
        //         "localhost"
        //     }
        // };



        Ok(())
    }


    fn extract_hostname_from_json_value(&self, obj: JsonValue) -> Result<(), PlatformError> {
        // Implement the logic here
        Ok(())
    }


}

