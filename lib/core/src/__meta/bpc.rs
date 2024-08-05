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
pub enum BpcAttributes {
    Enable,
    Voltage,
    Current
}

impl BpcAttributes {
    pub fn to_string(&self) -> String {
        match self {
            BpcAttributes::Enable => "enable".to_string(),
            BpcAttributes::Voltage => "voltage".to_string(),
            BpcAttributes::Current => "current".to_string(),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            BpcAttributes::Enable => "enable",
            BpcAttributes::Voltage => "voltage",
            BpcAttributes::Current => "current"
        }
    }

    pub fn all_attributes() -> Vec<String> {
        return vec![
            "enable".to_string(),
            "voltage".to_string(),
            "current".to_string()
        ]
    }
}

pub struct BpcParams {
    pub voltage_min: f64,
    pub voltage_max: f64,
    pub voltage_decimals: i32,

    pub current_min: f64,
    pub current_max: f64,
    pub current_decimals: i32,
}

#[async_trait]
pub trait BpcActions: Send + Sync {

    /// Initialize the interface
    /// The connector initialization must be done here
    ///
    async fn initializating(&mut self, interface: &AmInterface) -> PlatformFunctionResult;

    async fn read_enable_value(&mut self, interface: &AmInterface) -> Result<bool, PlatformError>;

    async fn write_enable_value(&mut self, interface: &AmInterface, v: bool) -> PlatformFunctionResult;

    async fn read_voltage_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError>;

    async fn write_voltage_value(&mut self, interface: &AmInterface, v: f64);

    async fn read_current_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError>;

    async fn write_current_value(&mut self, interface: &AmInterface, v: f64);


// async def _PZA_DRV_BPC_read_voltage_decimals(self):
//     """Must return the number of decimals supported for the voltage
//     """
//     raise NotImplementedError("Must be implemented !")

// # ---


// async def _PZA_DRV_BPC_current_value_min_max(self):
//     """Must return the current range of the power supply
//     """
//     return {"min": 0, "max": 0 }

// async def _PZA_DRV_BPC_read_current_decimals(self):
//     """Must return the number of decimals supported for the amperage
//     """
//     raise NotImplementedError("Must be implemented !")

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

struct BpcInterface {

    params: BpcParams,
    actions: Box<dyn BpcActions>
}
type AmBpcInterface = Arc<Mutex<BpcInterface>>;

impl BpcInterface {
    fn new(params: BpcParams, actions: Box<dyn BpcActions>) -> BpcInterface {
        return BpcInterface {
            params: params,
            actions: actions
        }
    }
    fn new_am(params: BpcParams, actions: Box<dyn BpcActions>) -> AmBpcInterface {
        return Arc::new(Mutex::new( BpcInterface::new(params, actions) ));
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

struct BpcStates {
    bpc_interface: Arc<Mutex<BpcInterface>>,
    attributes_used: Vec<String>
}


#[async_trait]
impl interface::fsm::States for BpcStates {

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
        let mut bpc_itf = self.bpc_interface.lock().await;

        // Custom initialization slot
        bpc_itf.actions.initializating(&interface).await?;

        // Get attribute name 
        let enable_attribute = BpcAttributes::Enable;
        let voltage_attribute = BpcAttributes::Voltage;
        let current_attribute = BpcAttributes::Current;

        // If enable is used by the interface 
        if self.attributes_used.contains(&enable_attribute.to_string()) {

            // Register enable attribute
            interface.lock().await.register_attribute(JsonAttribute::new_boxed(enable_attribute.as_str(), true));

            // Init enable
            let enable_value = bpc_itf.actions.read_enable_value(&interface).await?;
            interface.lock().await.update_attribute_with_bool("enable", "value", enable_value)?;
        }

        // If voltage attribute is used by interface 
        if self.attributes_used.contains(&voltage_attribute.to_string()) {
            let voltage_str = voltage_attribute.as_str();

            interface.lock().await.register_attribute(JsonAttribute::new_boxed(voltage_str, true));

            // Init voltage
            let voltage_value = bpc_itf.actions.read_voltage_value(&interface).await?;
            interface.lock().await.update_attribute_with_f64(voltage_str, "min", bpc_itf.params.voltage_min );
            interface.lock().await.update_attribute_with_f64(voltage_str, "max", bpc_itf.params.voltage_max );
            interface.lock().await.update_attribute_with_f64(voltage_str, "value", voltage_value);
            interface.lock().await.update_attribute_with_f64(voltage_str, "decimals", bpc_itf.params.voltage_decimals as f64);
            interface.lock().await.update_attribute_with_f64(voltage_str, "polling_cycle", 0.0);
        }

        // If current attribute is used by interface 
        if self.attributes_used.contains(&current_attribute.to_string()) { 
            let current_str = current_attribute.as_str();

            interface.lock().await.register_attribute(JsonAttribute::new_boxed(current_str, true));

             // Init current
            let current_value = bpc_itf.actions.read_current_value(&interface).await?;
            interface.lock().await.update_attribute_with_f64(current_str, "min", bpc_itf.params.current_min );
            interface.lock().await.update_attribute_with_f64(current_str, "max", bpc_itf.params.current_max );
            interface.lock().await.update_attribute_with_f64(current_str, "value", current_value);
            interface.lock().await.update_attribute_with_f64(current_str, "decimals", bpc_itf.params.current_decimals as f64);
            interface.lock().await.update_attribute_with_f64(current_str, "polling_cycle", 0.0);
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

const ID_ENABLE: subscription::Id = 0;
const ID_VOLTAGE: subscription::Id = 1;
const ID_CURRENT: subscription::Id = 2;

struct BpcSubscriber {
    bpc_interface: Arc<Mutex<BpcInterface>>,
    attributes_used: Vec<String>
}

impl BpcSubscriber {

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
        self.bpc_interface.lock().await
            .actions.write_enable_value(&interface, requested_value).await?;

        let r_value = self.bpc_interface.lock().await
            .actions.read_enable_value(&interface).await?;

        interface.lock().await
            .update_attribute_with_bool("enable", "value", r_value)?;
        
        Ok(())
    }

    /// 
    /// 
    #[inline(always)]
    async fn process_voltage_value(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, field_data: &Value)
        -> Result<(), PlatformError>
        {
        let requested_value = match field_data.as_f64(){
            Some(val) => val,
            None => return __platform_error_result!("Voltage value not provided")
        };
        self.bpc_interface.lock().await
            .actions.write_voltage_value(&interface, requested_value as f64).await;

        let r_value = self.bpc_interface.lock().await
            .actions.read_voltage_value(&interface).await?;

        interface.lock().await
            .update_attribute_with_f64("voltage", "value", r_value as f64);

        Ok(())
    }

    /// 
    /// 
    #[inline(always)]
    async fn process_current_value(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, field_data: &Value)
    -> Result<(), PlatformError>
    {
        let requested_value = match field_data.as_f64(){
            Some(val) => val,
            None => return __platform_error_result!("Unable to parse requested current as f64")
        };
        self.bpc_interface.lock().await
            .actions.write_current_value(&interface, requested_value as f64).await;

        let r_value = self.bpc_interface.lock().await
            .actions.read_current_value(&interface).await?;

        interface.lock().await
            .update_attribute_with_f64("current", "value", r_value as f64);

        Ok(())
    }


}

#[async_trait]
impl interface::subscriber::Subscriber for BpcSubscriber {

    /// Get the list of attributes names
    ///
    async fn attributes_names(&self) -> Vec<(subscription::Id, String)> {

        let mut attributes_names: Vec<(subscription::Id, String)> = Vec::new();

        if self.attributes_used.contains(&BpcAttributes::Enable.to_string()) {
            attributes_names.push((ID_ENABLE, BpcAttributes::Enable.to_string()));
        }
        if self.attributes_used.contains(&BpcAttributes::Voltage.to_string()) {
            attributes_names.push((ID_VOLTAGE, BpcAttributes::Voltage.to_string()));
        }
        if self.attributes_used.contains(&BpcAttributes::Current.to_string()) {
            attributes_names.push((ID_CURRENT, BpcAttributes::Current.to_string()));
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

                        println!("BpcSubscriber::process: {:?}", msg.topic());
                        println!("BpcSubscriber::process: {:?}", msg.payload());

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
                                // Go until contains only if enable if attribute_name == "enable" and field_name == "value"
                                if attribute_name == BpcAttributes::Enable.as_str() && field_name == "value" && self.attributes_used.contains(&BpcAttributes::Enable.to_string()) {
                                    self.process_enable_value(&interface, attribute_name, field_name, field_data).await?;
                                }
                                else if attribute_name == BpcAttributes::Voltage.as_str() && field_name == "value" && self.attributes_used.contains(&BpcAttributes::Voltage.to_string()) {
                                    self.process_voltage_value(interface, attribute_name, field_name, field_data).await?;
                                }
                                else if attribute_name == BpcAttributes::Current.as_str() && field_name == "value" && self.attributes_used.contains(&BpcAttributes::Current.to_string()) {
                                    self.process_current_value(interface, attribute_name, field_name, field_data).await?;
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
    params: BpcParams,
    actions: Box<dyn BpcActions>,
    attributes_used: Vec<String>
) -> InterfaceBuilder {

    let c = BpcInterface::new_am(params, actions);

    return InterfaceBuilder::new(
        name,
        "bpc",
        "0.0",
        Box::new(BpcStates{bpc_interface: c.clone(), attributes_used: attributes_used.clone()}),
        Box::new(BpcSubscriber{bpc_interface: c.clone(), attributes_used: attributes_used.clone()})
    );
}

