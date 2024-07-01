use async_trait::async_trait;
use panduza_core::Error as PlatformError;
use panduza_core::meta::blc;
use panduza_core::interface::AmInterface;
use panduza_core::interface::builder::Builder as InterfaceBuilder;

use panduza_connectors::serial::tty::{self, TtyConnector};
use panduza_connectors::serial::tty::Config as SerialConfig;

///
/// 
struct S0501BlcActions {
    connector_tty: tty::TtyConnector,
    serial_config: SerialConfig,
    mode_value: String,
    enable_value: bool,
    power_value: f64,
    current_value: f64,
    time_lock_duration: Option<tokio::time::Duration>,
}

#[async_trait]
impl blc::BlcActions for S0501BlcActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {

        self.connector_tty = tty::get(&self.serial_config).await.unwrap();
        self.connector_tty.init().await;

        let mut response: &mut [u8] = &mut [0; 1024];
        let _result = self.connector_tty.write_then_read(
            b"?\r",
            &mut response,
            self.time_lock_duration
        ).await
            .map(|nb_of_bytes| {
                let response_bytes = &response[0..nb_of_bytes];
                let response_string = String::from_utf8(response_bytes.to_vec()).unwrap();
                println!("S0501BlcActions - initializating: {}", response_string);
            });


        return Ok(());
    }

    /// Read the mode value
    /// 
    async fn read_mode_value(&mut self, interface: &AmInterface) -> Result<String, PlatformError> {

        let mut ok_value = false;
        while !ok_value {
            let mut response: &mut [u8] = &mut [0; 1024];
            let _result = self.connector_tty.write_then_read(
                b"gam?\r",
                &mut response,
                self.time_lock_duration
            ).await
                .map(|nb_of_bytes| {
                    let mode_b = &response[0..nb_of_bytes];

                    println!("r mode {:?} !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!", String::from_utf8(mode_b.to_vec()).unwrap());
                    let mode_s = String::from_utf8(mode_b.to_vec()).unwrap()
                        .trim().to_string(); // Remove \r\n form the message before parsing
                        // .parse::<u16>().unwrap();

                    
                    match mode_s.parse::<u16>() {
                        Ok(u) => {
                            ok_value = true;
                            self.mode_value = match u {
                                0 => "constant_current".to_string(),
                                1 => "constant_power".to_string(),
                                _ => "no_regulation".to_string()
                            };
                        }
                        Err(e) => {
                            ok_value = false;
                            println!("Failed to parse {} : {}", mode_s, e); 
                        }
                    };

                    // match mode_s.as_str() {
                    //     "\r\n" => { ok_value = "\r\n".to_string() }
                    //     _ => { ok_value = mode_s.clone() }
                    // }

                    // self.mode_value = match mode_s.parse::<u16>().unwrap() {
                    //     0 => "constant_current".to_string(),
                    //     1 => "constant_power".to_string(),
                    //     _ => "no_regulation".to_string()
                    // };
                });
        }

        interface.lock().await.log_info(
            format!("read mode value : {}", self.mode_value.clone())
        );
        return Ok(self.mode_value.clone());
    }

    /// Write the mode value
    /// 
    async fn write_mode_value(&mut self, interface: &AmInterface, v: String) {

        interface.lock().await.log_info(
            format!("write mode value : {}", v)
        );

        let command = match v.as_str() {
            "constant_current" => format!("ci\r"),
            "constant_power" => format!("cp\r"),
            _ => return
        };

        // let _result = self.connector_tty.write(
        //     command.as_bytes(),
        //     self.time_lock_duration
        // ).await
        //     .map(|_nb_of_bytes| {
        //     });
        
        // Clean the buffer from previous values
        let mut ok_val = String::new();

        while ok_val != "OK".to_string() {
            let mut response: &mut [u8] = &mut [0; 1024];
            let _result = self.connector_tty.write_then_read(
                command.as_bytes(), // b"gam?\r",
                &mut response,
                self.time_lock_duration
            ).await
                .map(|nb_of_bytes| {
                    let value_b = &response[0..nb_of_bytes];

                    println!("w mode {:?} ???????????????????????????????????", String::from_utf8(value_b.to_vec()).unwrap());
                    let values = String::from_utf8(value_b.to_vec()).unwrap()
                        .trim().to_string();

                    for val in values.split("\r\n") {
                        match val {
                            "OK" => { ok_val = "OK".to_string() }
                            _ => { continue; }
                        }
                    };
                });
        }
    }

     /// Read the enable value
    /// 
    async fn read_enable_value(&mut self, interface: &AmInterface) -> Result<bool, PlatformError> {

        let mut ok_value = false;
        while !ok_value {
            let mut response: &mut [u8] = &mut [0; 1024];

            let _result = self.connector_tty.write_then_read(
                b"l?\r",
                &mut response,
                self.time_lock_duration
            ).await
                .map(|nb_of_bytes| {
                    let value_b = &response[0..nb_of_bytes];

                    let value_s = String::from_utf8(value_b.to_vec()).unwrap()
                        .trim().to_string(); // Remove \r\n form the message before parsing
                        // .parse::<u16>().unwrap();

                    
                    match value_s.parse::<u16>() {
                        Ok(u) => {
                            ok_value = true;
                            self.enable_value = match u {
                                0 => false,
                                _ => true
                            };
                        }
                        Err(e) => {
                            ok_value = false;
                            println!("Failed to parse {} : {}", value_s, e); 
                        }
                    };

                    // let value_i = String::from_utf8(value_b.to_vec()).unwrap()
                    //     .trim().to_string() // Remove \r\n form the message before parsing
                    //     .parse::<u16>().unwrap();

                    // self.enable_value = match value_i {
                    //     0 => false,
                    //     _ => true
                    // };
                });
        }

        interface.lock().await.log_info(
            format!("read enable value : {}", self.enable_value)
        );

        return Ok(self.enable_value);
    }

    /// Write the enable value
    /// 
    async fn write_enable_value(&mut self, interface: &AmInterface, v: bool) {

        let val_int = match v {
            true => 1,
            false => 0
        };

        let command = format!("l{}\r", val_int);

        interface.lock().await.log_info(
            format!("write enable value : {}", v)
        );

        // let _result = self.connector_tty.write(
        //     command.as_bytes(),
        //     self.time_lock_duration
        // ).await
        //     .map(|nb_of_bytes| {
        //         println!("nb of bytes: {:?}", nb_of_bytes);
        //     });
        
        // Clean the buffer from previous values
        let mut ok_val = String::new();

        while ok_val != "OK".to_string() {
            let mut response: &mut [u8] = &mut [0; 1024];
            let _result = self.connector_tty.write_then_read(
                command.as_bytes(), // b"l?\r",
                &mut response,
                self.time_lock_duration
            ).await
                .map(|nb_of_bytes| {
                    let value_b = &response[0..nb_of_bytes];
                    let values = String::from_utf8(value_b.to_vec()).unwrap();

                    // If multiple messages are flushed at once, splits the result to check every messages
                    for val in values.split("\r\n") {
                        match val {
                            "OK" => { ok_val = "OK".to_string() }
                            _ => { continue; }
                        }
                    };
                });
        }

        // The laser has an intertia to change to from OFF to ON so waits until it actually change state
        let mut value_i: u16 = 5; // The returned value is 0 or 1 so 5 is sure to be out of range

        while value_i != val_int {
            let mut response: &mut [u8] = &mut [0; 1024];
            let _result = self.connector_tty.write_then_read(
                b"l?\r",
                &mut response,
                self.time_lock_duration
            ).await
                .map(|nb_of_bytes| {
                    let value_b = &response[0..nb_of_bytes];
                    let values = String::from_utf8(value_b.to_vec()).unwrap();

                    // If multiple messages are flushed at once, splits the result to check every messages
                    for val in values.split("\r\n") {
                        match val.parse::<u16>() {
                            Ok(u) => { value_i = u }
                            Err(_e) => { continue; }
                        }
                    };
                });
        }
    }

    /// Read the power value
    /// 
    async fn read_power_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        let mut ok_value = false;
        while !ok_value {
            let mut response: &mut [u8] = &mut [0; 1024];
            let _result = self.connector_tty.write_then_read(
                b"p?\r",
                &mut response,
                self.time_lock_duration
            ).await
                .map(|nb_of_bytes| {
                    let power_b = &response[0..nb_of_bytes];
                    let power_s = String::from_utf8(power_b.to_vec()).unwrap()
                        .trim().to_string(); // Remove \r\n form the message before parsing
                    
                    match power_s.parse::<f64>() {
                        Ok(f) => {
                            ok_value = true;
                            self.power_value = f;
                        }
                        Err(e) => {
                            ok_value = false;
                            println!("Failed to parse {} : {}", power_s, e); 
                        }
                    };

                    // self.power_value = String::from_utf8(power_b.to_vec()).unwrap()
                    //     .trim().to_string() // Remove \r\n form the message before parsing
                    //     .parse::<f64>().unwrap();
                });
        }

        interface.lock().await.log_info(
            format!("read power : {}", self.power_value)
        );

        return Ok(self.power_value);
    }

    /// Write the power value
    /// 
    async fn write_power_value(&mut self, interface: &AmInterface, v: f64) {
        
        interface.lock().await.log_info(
            format!("write power : {}", v)
        );

        let command = format!("p {}\r", v);

        let _result = self.connector_tty.write(
            command.as_bytes(),
            self.time_lock_duration
        ).await
            .map(|_nb_of_bytes| {
            });

            // let mut response: &mut [u8] = &mut [0; 1024];
            // let _result = self.connector_tty.read(
            //     &mut response,
            // ).await
            //     .map(|nb_of_bytes| {
            //         let value_b = &response[0..nb_of_bytes];
            //         let values = String::from_utf8(value_b.to_vec()).unwrap();

                    // // If multiple messages are flushed at once, splits the result to check every messages
                    // for val in values.split("\r\n") {
                    //     match val {
                    //         "OK" => { ok_val = "OK".to_string() }
                    //         _ => { continue; }
                    //     }
                    // };
                // });

        // Clean the buffer from previous values
        let mut ok_val = String::new();

        while ok_val != "OK".to_string() {
            let mut response: &mut [u8] = &mut [0; 1024];
            let _result = self.connector_tty.write_then_read(
                command.as_bytes(), // b"p?\r",
                &mut response,
                self.time_lock_duration
            ).await
                .map(|nb_of_bytes| {
                    let value_b = &response[0..nb_of_bytes];
                    let values = String::from_utf8(value_b.to_vec()).unwrap();

                    // If multiple messages are flushed at once, splits the result to check every messages
                    for val in values.split("\r\n") {
                        match val {
                            "OK" => { ok_val = "OK".to_string() }
                            _ => { continue; }
                        }
                    };
                });
        }
    }

    /// Read the current value
    /// 
    async fn read_current_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        let mut ok_value = false;
        while !ok_value {
            let mut response: &mut [u8] = &mut [0; 1024];
            let _result = self.connector_tty.write_then_read(
                b"glc?\r",
                &mut response,
                self.time_lock_duration
            ).await
                .map(|nb_of_bytes| {
                    let current_b = &response[0..nb_of_bytes];

                    let current_s = String::from_utf8(current_b.to_vec()).unwrap()
                        .trim().to_string(); // Remove \r\n form the message before parsing
                    
                    match current_s.parse::<f64>() {
                        Ok(f) => {
                            ok_value = true;
                            self.current_value = f;
                        }
                        Err(e) => {
                            ok_value = false;
                            println!("Failed to parse {} : {}", current_s, e); 
                        }
                    };

                    // self.current_value = String::from_utf8(current_b.to_vec()).unwrap()
                    //     .trim().to_string() // Remove \r\n form the message before parsing
                    //     .parse::<f64>().unwrap();
                });
        }

        interface.lock().await.log_info(
            format!("read current : {}", self.current_value)
        );

        return Ok(self.current_value);
    }

    /// Write the current value
    /// 
    async fn write_current_value(&mut self, interface: &AmInterface, v: f64) {
        interface.lock().await.log_info(
            format!("write current : {}", v)
        );

        let command = format!("slc {}\r", v);

        // let _result = self.connector_tty.write(
        //     command.as_bytes(),
        //     self.time_lock_duration
        // ).await
        //     .map(|_nb_of_bytes| {
        //     });

        // Clean the buffer from previous values
        let mut ok_val = String::new();

        while ok_val != "OK".to_string() {
            let mut response: &mut [u8] = &mut [0; 1024];
            let _result = self.connector_tty.write_then_read(
                command.as_bytes(), // b"glc?\r",
                &mut response,
                self.time_lock_duration
            ).await
                .map(|nb_of_bytes| {
                    let value_b = &response[0..nb_of_bytes];
                    let values = String::from_utf8(value_b.to_vec()).unwrap();

                    // If multiple messages are flushed at once, splits the result to check every messages
                    for val in values.split("\r\n") {
                        match val {
                            "OK" => { ok_val = "OK".to_string() }
                            _ => { continue; }
                        }
                    };
                });
        }
    }
}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A,
    serial_config: &SerialConfig
) -> InterfaceBuilder {

    return blc::build(
        name, 
        blc::BlcParams {
            power_min: 0.0,
            power_max: 0.3,
            power_decimals: 3,

            current_min: 0.0,
            current_max: 0.5,
            current_decimals: 1,
        }, 
        Box::new(S0501BlcActions {
            connector_tty: TtyConnector::new(None),
            serial_config: serial_config.clone(),
            mode_value: "constant_power".to_string(),
            enable_value: false,
            power_value: 0.0,
            current_value: 0.0,
            time_lock_duration: Some(tokio::time::Duration::from_millis(100)),
        })
    )
}