use async_trait::async_trait;
use rusb;
use std::time::Duration;

// use rust_usbtmc::instrument::Instrument;
use crate::platform::PlatformError;
use crate::meta::powermeter;
use crate::interface::AmInterface;
use crate::interface::builder::Builder as InterfaceBuilder;


// use crate::connector::serial::tty::Tty;
// use crate::connector::serial::tty::{self, TtyConnector};
// use crate::connector::serial::tty::Config as SerialConfig;
// use crate::platform_error_result;


// static VID: u16 = 0x1313;
// static PID: u16 = 0x8079;


///
/// 
struct PM100APowermeterActions {
    // connector_tty: tty::TtyConnector,
    // serial_config: SerialConfig,
    // instrument: Instrument,
    measure_value: f64,
    // time_lock_duration: Option<tokio::time::Duration>,
}

#[async_trait]
impl powermeter::PowermeterActions for PM100APowermeterActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {

        // let c = rusb::Context::new();
        let d = rusb::devices();
        // println!("{}", d.unwrap().len());

        let ll = d.as_ref().unwrap();
        for dev in ll.iter() {
            println!("{:#04x}", dev.device_descriptor().unwrap().vendor_id());
            if dev.device_descriptor().unwrap().vendor_id() == 0x1313 {
                let usb_device = dev.open().unwrap();
                let mut data: [u8; 64] = [0; 64]; // Initialize the array with 64 zeros
                // Copy the string "*idn?" into the array
                data[.."*idn?".len()].copy_from_slice("*idn?".as_bytes());
                // Print the entire array (including null terminator) for demonstration purposesprintln!("{:?}", data);
                let count = usb_device.write_bulk(0x02, &data, Duration::from_millis(100));
                match count {
                    Ok(v) => println!("usize {}", v),
                    Err(e) => println!("Error: {}", e),
                };
                // println!("{}", count);
                    // match dev.open() {
                    //     Ok(h) => match h.read_languages(timeout) {
                    //         Ok(l) => {
                    //             if l.len() > 0 {
                    //                 Some(UsbDevice {
                    //                     handle: h,
                    //                     language: l[0],
                    //                     timeout,
                    //                 })
                    //             } else {
                    //                 None
                    //             }
                    //         }
                    //         Err(_) => None,
                    //     },
                    //     Err(_) => None,
                    // }
            }
        }

        
        // for dev in d.unwrap() {
        //     println!("{}", dev);
        // }
        

        // self.connector_tty = tty::get(&self.serial_config).await.unwrap();
        // self.connector_tty.init().await;
        // self.instrument = Instrument::new(VID, PID);

        // println!("yooooo!");
        // println!("Ask: {}", self.instrument.ask("*IDN?").unwrap());
        // self.measure_value = self.instrument.read().unwrap().parse::<f64>().unwrap();
        // println!("measure {}", self.measure_value);

        return Ok(());
    }

    /// Read the measure value
    /// 
    async fn read_measure_value(&mut self, _interface: &AmInterface) -> Result<f64, PlatformError> {

        // self.measure_value = self.instrument.read().unwrap().parse::<f64>().unwrap();
        // println!("measure {}", self.measure_value);

        // interface.lock().await.log_info(
        //     format!("PM100A - read_measure_value: {}", self.measure_value)
        // );
        return Ok(self.measure_value);

        // interface.lock().await.log_warn(
        //     format!("NOT IMPLEMENTED PM100A - read_measure_value: {}", self.measure_value)
        // );
        // return Ok(self.measure_value);
    }
}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A,
    // serial_config: &SerialConfig
) -> InterfaceBuilder {

    return powermeter::build(
        name, 
        powermeter::PowermeterParams {
            measure_decimals: 5,
        }, 
        Box::new(PM100APowermeterActions {
            // connector_tty: TtyConnector::new(None),
            // serial_config: serial_config.clone(),
            // instrument: Instrument::new(0, 0),
            measure_value: 0.0,
            // time_lock_duration: Some(tokio::time::Duration::from_millis(100)),
        })
    )
}

