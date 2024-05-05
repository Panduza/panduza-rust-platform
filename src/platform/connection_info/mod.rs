
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;
use std::fs::File;
use std::io::Read;

use crate::platform::PlatformError;

mod tests;


#[derive(Debug)]
enum ErrorType {
    // COVER:REQ_CONN_INFO_0030_00
    ContentBadFormat,
    // COVER:REQ_CONN_INFO_0040_00
    MandatoryFieldMissing,
    // COVER:REQ_CONN_INFO_0050_00
    FileDoesNotExist,
}


#[derive(Debug)]
pub struct Error {

    type_: ErrorType,

    message: String,
}

impl Error {

    fn new(type_: ErrorType, message: &str) -> Self {
        Self {
            type_,
            message: message.to_string(),
        }
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn type_(&self) -> &ErrorType {
        &self.type_
    }
}

fn ContentBadFormatError(message: &str) -> Error {
    Error::new(ErrorType::ContentBadFormat, message)
}
fn MandatoryFieldMissingError(message: &str) -> Error {
    Error::new(ErrorType::MandatoryFieldMissing, message)
}
fn FileDoesNotExistError(message: &str) -> Error {
    Error::new(ErrorType::FileDoesNotExist, message)
}



#[derive(Debug, PartialEq, Eq)]
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
    fn build_from_json_value(json_obj: JsonValue) -> Result<Self, Error> {
        json_obj.as_object()
            .ok_or(ContentBadFormatError( "Except a JSON object at file root"))
            .and_then(ConnectionInfo::build_from_map_object)
    }

    /// Create a new ConnectionInfo object from a JSON map object
    ///
    fn build_from_map_object(map_obj: &JsonMap<String, JsonValue>) -> Result<Self, Error> {

        
        let host = map_obj.get("host")
            .ok_or(MandatoryFieldMissingError("[host] section must be provided"))?;

        
        // let hostname = 
        //         .and_then(|v| v.as_object() )
        
        
        //     map_obj.get("broker_host")
        //         .ok_or(err!(MandatoryFieldMissing, "broker_host")
        //         .and_then(|v| v.as_str())
        //         .unwrap_or("localhost");

        // let fallback_port = || {
        //     Some(JsonValue::from(1883))
        // };

        let port = 
            map_obj.get("broker_port")
                // .or_else(fallback_port)
                .and_then(|v| v.as_u64())
                .unwrap_or(1883);
    
        Ok(
            Self {
                hostname: hostname.to_string(),
                port: port as u16,
            }
        )
    }

    /// Getter Hostname
    ///
    pub fn hostname(&self) -> &String {
        &self.hostname
    }

    /// Getter Port
    ///
    pub fn port(&self) -> u16 {
        self.port
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

