use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::Mutex;
use tokio::time::{self, sleep};

use crate::attribute::JsonAttribute;
use crate::interface::{AmInterface, ThreadSafeInterface};

use crate::{interface, subscription};
use crate::interface::builder::Builder as InterfaceBuilder;


use crate::Error as PlatformError;
use crate::__platform_error_result;

use crate::FunctionResult as PlatformFunctionResult;


// Enum of every attributes who can be used by a bench power controller
pub enum BlcAttributes {
    Mode,
    Enable,
    Power,
    Current,
    AnalogModulation
}

impl BlcAttributes {
    pub fn to_string(&self) -> String {
        match self {
            BlcAttributes::Mode => "mode".to_string(),
            BlcAttributes::Enable => "enable".to_string(),
            BlcAttributes::Power => "power".to_string(),
            BlcAttributes::Current => "current".to_string(),
            BlcAttributes::AnalogModulation => "analog_modulation".to_string()
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            BlcAttributes::Mode => "mode",
            BlcAttributes::Enable => "enable",
            BlcAttributes::Power => "power",
            BlcAttributes::Current => "current",
            BlcAttributes::AnalogModulation => "analog_modulation"
        }
    }

    pub fn all_attributes() -> Vec<String> {
        return vec![
            "mode".to_string(),
            "enable".to_string(),
            "power".to_string(),
            "current".to_string(),
            "analog_modulation".to_string()
        ]
    }
}

pub struct BlcParams {
    pub power_min: f64,
    pub power_decimals: i32,

    pub current_min: f64,
    pub current_decimals: i32,
}

#[async_trait]
pub trait BlcActions: Send + Sync {

    /// Initialize the interface
    /// The connector initialization must be done here
    ///
    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError>;

    async fn read_mode_value(&mut self, interface: &AmInterface) -> Result<String, PlatformError>;

    async fn write_mode_value(&mut self, interface: &AmInterface, v: String) -> Result<(), PlatformError>;

    async fn read_enable_value(&mut self, interface: &AmInterface) -> Result<bool, PlatformError>;

    async fn write_enable_value(&mut self, interface: &AmInterface, v: bool) -> Result<(), PlatformError>;

    async fn read_power_max(&mut self, interface: &AmInterface) -> Result<f64, PlatformError>;

    async fn read_power_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError>;

    async fn write_power_value(&mut self, interface: &AmInterface, v: f64) -> Result<(), PlatformError>;

    async fn read_max_current_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError>;

    async fn read_current_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError>;

    async fn write_current_value(&mut self, interface: &AmInterface, v: f64) -> Result<(), PlatformError>;

    async fn read_analog_modulation(&mut self, interface: &AmInterface) -> Result<bool, PlatformError>;

    async fn write_analog_modulation(&mut self, interface: &AmInterface, v: bool) -> Result<(), PlatformError>;
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------


// pub struct EnableAttribute {
//     attr: JsonAttribute,
// }

// pub struct F32ValueAttribute {
//     attr: JsonAttribute,
// }


// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

struct BlcInterface {

    params: BlcParams,
    actions: Box<dyn BlcActions>
}
type AmBlcInterface = Arc<Mutex<BlcInterface>>;

impl BlcInterface {
    fn new(params: BlcParams, actions: Box<dyn BlcActions>) -> BlcInterface {
        return BlcInterface {
            params: params,
            actions: actions
        }
    }
    fn new_am(params: BlcParams, actions: Box<dyn BlcActions>) -> AmBlcInterface {
        return Arc::new(Mutex::new( BlcInterface::new(params, actions) ));
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

struct BlcStates {
    blc_interface: Arc<Mutex<BlcInterface>>,
    attributes_used: Vec<String>
}


#[async_trait]
impl interface::fsm::States for BlcStates {

    /// Just wait for an fsm event for the connection
    ///
    async fn connecting(&self, interface: &AmInterface)
    {
        interface::basic::wait_for_fsm_event(interface).await;
    }

    /// Initialize the interface
    ///
    async fn initializating(&self, interface: &AmInterface)
    -> Result<(), PlatformError>
    {
        let mut blc_itf = self.blc_interface.lock().await;

        // Custom initialization slot
        blc_itf.actions.initializating(&interface).await?;

        // If analog modulation attribute is used by device interface (must be put 
        // at 1 at first to init other attribute else read command could not be send 
        // correctly)
        if self.attributes_used.contains(&BlcAttributes::AnalogModulation.to_string()) {
            interface.lock().await.register_attribute(JsonAttribute::new_boxed(BlcAttributes::AnalogModulation.as_str(), true));

            // Init analog modulation (should disable analog modulation)
            blc_itf.actions.write_analog_modulation(&interface, true).await?;
            let analog_modulation = blc_itf.actions.read_analog_modulation(&interface).await?;

            interface.lock().await.update_attribute_with_bool(BlcAttributes::AnalogModulation.as_str(), "value", analog_modulation)?;
        }   

        // If mode power/current attribute is used by device interface
        if self.attributes_used.contains(&BlcAttributes::Mode.to_string()) {
            interface.lock().await.register_attribute(JsonAttribute::new_boxed(BlcAttributes::Mode.as_str(), true));

            // Init mode
            let mode_value = blc_itf.actions.read_mode_value(&interface).await?;
            interface.lock().await.update_attribute_with_string(BlcAttributes::Mode.as_str(), "value", &mode_value);
        }

        // If mode enable attribute is used by device interface
        if self.attributes_used.contains(&BlcAttributes::Enable.to_string()) {
            interface.lock().await.register_attribute(JsonAttribute::new_boxed(BlcAttributes::Enable.as_str(), true));

            // Init enable
            let enable_value = blc_itf.actions.read_enable_value(&interface).await?;
            
            let _update_att = interface.lock().await.update_attribute_with_bool(BlcAttributes::Enable.as_str(), "value", enable_value)?;
        }

        // If power attribute is used by device interface
        if self.attributes_used.contains(&BlcAttributes::Power.to_string()) {
            let power_attribute_str = BlcAttributes::Power.as_str();

            interface.lock().await.register_attribute(JsonAttribute::new_boxed(power_attribute_str, true));

            // Init power
            let max_power = blc_itf.actions.read_power_max(&interface).await?;
            let power_value = blc_itf.actions.read_power_value(&interface).await?;
            interface.lock().await.update_attribute_with_f64(power_attribute_str, "min", blc_itf.params.power_min );
            interface.lock().await.update_attribute_with_f64(power_attribute_str, "max", max_power);
            interface.lock().await.update_attribute_with_f64(power_attribute_str, "value", power_value);
            interface.lock().await.update_attribute_with_f64(power_attribute_str, "decimals", blc_itf.params.power_decimals as f64);
            interface.lock().await.update_attribute_with_f64(power_attribute_str, "polling_cycle", 0.0);
        }

        // If current attribute is used by device interface
        if self.attributes_used.contains(&BlcAttributes::Current.to_string()) {
            let current_attribute_str = BlcAttributes::Current.as_str();

            interface.lock().await.register_attribute(JsonAttribute::new_boxed(current_attribute_str, true));

            // Init current
            let max_current = blc_itf.actions.read_max_current_value(&interface).await?;
            let current_value = blc_itf.actions.read_current_value(&interface).await?;
            interface.lock().await.update_attribute_with_f64(current_attribute_str, "min", blc_itf.params.current_min );
            interface.lock().await.update_attribute_with_f64(current_attribute_str, "max", max_current );
            interface.lock().await.update_attribute_with_f64(current_attribute_str, "value", current_value);
            interface.lock().await.update_attribute_with_f64(current_attribute_str, "decimals", blc_itf.params.current_decimals as f64);
            interface.lock().await.update_attribute_with_f64(current_attribute_str, "polling_cycle", 0.0);
        }

        // Publish all attributes for start
        interface.lock().await.publish_all_attributes().await;

        // Notify the end of the initialization
        interface.lock().await.set_event_init_done();

        Ok(())
    }

    async fn running(&self, interface: &AmInterface)
    {
        println!("running");


        interface::basic::wait_for_fsm_event(interface).await;
    }

    async fn warning(&self, interface: &ThreadSafeInterface)
    {
        // Wait for 5 sec and reboot
        sleep(Duration::from_secs(5)).await;

        interface.lock().await.set_event_reboot();

        println!("warning");
    }

    async fn cleaning(&self, interface: &ThreadSafeInterface)
    {
        println!("cleaning");
        interface.lock().await.set_event_cleaned();
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

const ID_MODE: subscription::Id = 0;
const ID_ENABLE: subscription::Id = 1;
const ID_POWER: subscription::Id = 2;
const ID_CURRENT: subscription::Id = 3;

struct BlcSubscriber {
    blc_interface: Arc<Mutex<BlcInterface>>,
    attributes_used: Vec<String>
}

impl BlcSubscriber {

    /// 
    /// 
    #[inline(always)]
    async fn process_analog_modulation(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, field_data: &Value)
    -> Result<(), PlatformError>
    {
        
        let requested_value = match field_data.as_bool() {
            Some(request) => request,
            None => return __platform_error_result!("Analog modulation value not provided")
        };
        self.blc_interface.lock().await
            .actions.write_analog_modulation(&interface, requested_value).await?;

        let r_value = self.blc_interface.lock().await
            .actions.read_analog_modulation(&interface).await?;

        interface.lock().await
            .update_attribute_with_bool(&BlcAttributes::AnalogModulation.to_string(), "value", r_value)?;

        Ok(())
    }

    /// 
    /// 
    #[inline(always)]
    async fn process_mode_value(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, field_data: &Value)
    -> Result<(), PlatformError>
    {
        let requested_value = match field_data.as_str() {
            Some(str) => str,
            None => return __platform_error_result!("Mode value not provided")
        }.to_string();
        self.blc_interface.lock().await
            .actions.write_mode_value(&interface, requested_value).await?;

        let r_value = self.blc_interface.lock().await
            .actions.read_mode_value(&interface).await?;

        interface.lock().await
            .update_attribute_with_string("mode", "value", &r_value);

        Ok(())
    }

    /// 
    /// 
    #[inline(always)]
    async fn process_enable_value(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, field_data: &Value)
    -> Result<(), PlatformError>
    {
        let requested_value = match field_data.as_bool() {
            Some(bool) => bool,
            None => return __platform_error_result!("Enable value not provided")
        };
        self.blc_interface.lock().await
            .actions.write_enable_value(&interface, requested_value).await?;

        let r_value = self.blc_interface.lock().await
            .actions.read_enable_value(&interface).await?;

        interface.lock().await
            .update_attribute_with_bool("enable", "value", r_value)?;

        Ok(())
    }

    /// 
    /// 
    #[inline(always)]
    async fn process_power_value(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, field_data: &Value)
    -> Result<(), PlatformError>
    {
        let requested_value = match field_data.as_f64() {
            Some(val) => val,
            None => return __platform_error_result!("Power value not porvided")
        };
        self.blc_interface.lock().await
            .actions.write_power_value(&interface, requested_value as f64).await?;

            let r_value = self.blc_interface.lock().await
            .actions.read_power_value(&interface).await?;

        interface.lock().await
            .update_attribute_with_f64("power", "value", r_value as f64);

        Ok(())
    }

    /// 
    /// 
    #[inline(always)]
    async fn process_current_value(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, field_data: &Value)
    -> Result<(), PlatformError>
    {
        let requested_value = match field_data.as_f64() {
            Some(val) => val,
            None => return __platform_error_result!("Current value not porvided")
        };
        self.blc_interface.lock().await
            .actions.write_current_value(&interface, requested_value as f64).await?;

        let r_value = self.blc_interface.lock().await
            .actions.read_current_value(&interface).await?;

        interface.lock().await
            .update_attribute_with_f64("current", "value", r_value as f64);

        Ok(())
    }


}

#[async_trait]
impl interface::subscriber::Subscriber for BlcSubscriber {

    /// Get the list of attributes names
    ///
    async fn attributes_names(&self) -> Vec<(subscription::Id, String)> {

        let mut attributes_names: Vec<(subscription::Id, String)> = Vec::new();

        if self.attributes_used.contains(&BlcAttributes::Mode.to_string()) {
            attributes_names.push((ID_MODE, BlcAttributes::Mode.to_string()));
        }

        if self.attributes_used.contains(&BlcAttributes::Enable.to_string()) {
            attributes_names.push((ID_ENABLE, BlcAttributes::Enable.to_string()));
        }
        
        if self.attributes_used.contains(&BlcAttributes::Power.to_string()) {
            attributes_names.push((ID_POWER, BlcAttributes::Power.to_string()));
        }

        if self.attributes_used.contains(&BlcAttributes::Current.to_string()) {
            attributes_names.push((ID_CURRENT, BlcAttributes::Current.to_string()));
        }

        return attributes_names;
    }




    /// Process a message
    ///
    async fn process(&self, interface: &AmInterface, msg: &subscription::Message) -> PlatformFunctionResult {
        // Common processing
        interface::basic::process(&interface, msg).await;

        match msg {
            subscription::Message::Mqtt(msg) => {
                match msg.id() {
                subscription::ID_PZA_CMDS_SET => {
                    // interface.lock().await.publish_info().await;

                    // only when running state

                    println!("BlcSubscriber::process: {:?}", msg.topic());
                    println!("BlcSubscriber::process: {:?}", msg.payload());

                    let payload = msg.payload();
                    let oo = match serde_json::from_slice::<Value>(payload) {
                        Ok(val) => val,
                        Err(_e) => return __platform_error_result!("Unable to deserializa data")
                    };
                    let o = match oo.as_object() {
                        Some(val) => val,
                        None => return __platform_error_result!("No data provided")
                    };


                    for (attribute_name, fields) in o.iter() {
                        let fields_obj = match fields.as_object() {
                            Some(val) => val,
                            None => return __platform_error_result!("No data provided")
                        };
                        for (field_name, field_data) in fields_obj.iter() {
                            if attribute_name == BlcAttributes::Mode.as_str() && field_name == "value" && self.attributes_used.contains(&BlcAttributes::Mode.to_string()) {
                                self.process_mode_value(&interface, attribute_name, field_name, field_data).await?;
                            }
                            else if attribute_name == BlcAttributes::Enable.as_str() && field_name == "value" && self.attributes_used.contains(&BlcAttributes::Enable.to_string()) {
                                self.process_enable_value(&interface, attribute_name, field_name, field_data).await?;
                            }
                            else if attribute_name == BlcAttributes::Power.as_str() && field_name == "value" && self.attributes_used.contains(&BlcAttributes::Power.to_string()) {
                                self.process_power_value(interface, attribute_name, field_name, field_data).await?;
                            }
                            else if attribute_name == BlcAttributes::Current.as_str() && field_name == "value" && self.attributes_used.contains(&BlcAttributes::Current.to_string()) {
                                self.process_current_value(interface, attribute_name, field_name, field_data).await?;
                            }
                            else if attribute_name == BlcAttributes::AnalogModulation.as_str() && field_name == "value" && self.attributes_used.contains(&BlcAttributes::AnalogModulation.to_string()) {
                                self.process_analog_modulation(interface, attribute_name, field_name, field_data).await?;
                            }
                        }
                    }
                    interface.lock().await.publish_all_attributes().await;


                },
                    _ => {
                        // not managed by the common level
                    }
                }
            }
            _ => {
                // not managed by the common level
            }
        }

        Ok(())
    }


}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Build the meta interface for a Bench Power Channel
///
pub fn build<A: Into<String>>(
    name: A,
    params: BlcParams,
    actions: Box<dyn BlcActions>,
    attributes_used: Vec<String>
) -> InterfaceBuilder {

    let c = BlcInterface::new_am(params, actions);

    return InterfaceBuilder::new(
        name,
        "blc",
        "0.0",
        Box::new(BlcStates{blc_interface: c.clone(), attributes_used: attributes_used.clone()}),
        Box::new(BlcSubscriber{blc_interface: c.clone(), attributes_used: attributes_used.clone()})
    );
}

