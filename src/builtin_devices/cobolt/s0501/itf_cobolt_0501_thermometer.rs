use async_trait::async_trait;

use panduza_core::Error as PlatformError;
use panduza_core::meta::thermometer;
use panduza_core::interface::AmInterface;
use panduza_core::interface::builder::Builder as InterfaceBuilder;

use panduza_connectors::serial::tty::{self, TtyConnector};
use panduza_connectors::serial::tty::Config as SerialConfig;

/// Fake Thermometer Channel Data
/// 
struct S0501ThermometerActions {
    connector_tty: tty::TtyConnector,
    serial_config: SerialConfig,
    measure_value: f64,
    time_lock_duration: Option<tokio::time::Duration>,
}

#[async_trait]
impl thermometer::ThermometerActions for S0501ThermometerActions {
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
                println!("S0501ThermometerActions - initializating: {}", response_string);
            });


        return Ok(());
    }

    /// Read the measure value
    /// 
    async fn read_measure_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {
        let mut ok_value = false;
        while !ok_value {
            let mut response: &mut [u8] = &mut [0; 1024];
            let _result = self.connector_tty.write_then_read(
                b"rtec4t?\r",
                &mut response,
                self.time_lock_duration
            ).await
                .map(|nb_of_bytes| {
                    let measure_b = &response[0..nb_of_bytes];

                    let measure_s = String::from_utf8(measure_b.to_vec()).unwrap()
                        .trim().to_string(); // Remove \r\n form the message before parsing
                    
                    match measure_s.parse::<f64>() {
                        Ok(f) => {
                            ok_value = true;
                            self.measure_value = f;
                        }
                        Err(e) => {
                            ok_value = false;
                            println!("Failed to parse {} : {}", measure_s, e); 
                        }
                    };
                    
                    // self.measure_value = String::from_utf8(power_b.to_vec()).unwrap()
                    //     .trim().to_string() // Remove \r\n form the message before parsing
                    //     .parse::<f64>().unwrap();
                });
        }

        // interface.lock().await.log_info(
        //     format!("S0501Thermometer - read_measure_value: {}", self.measure_value)
        // );

        return Ok(self.measure_value);
    }
}



/// Interface to emulate a Thermometer Channel
/// 
pub fn build<A: Into<String>>(
    name: A,
    serial_config: &SerialConfig
) -> InterfaceBuilder {

    return thermometer::build(
        name, 
        thermometer::ThermometerParams {
            measure_decimals: 3
        },
        Box::new(S0501ThermometerActions {
            connector_tty: TtyConnector::new(None),
            serial_config: serial_config.clone(),
            measure_value: 0.0,
            time_lock_duration: Some(tokio::time::Duration::from_millis(100)),
        })
    )
}

