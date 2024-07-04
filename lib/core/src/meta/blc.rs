use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::Mutex;

use crate::attribute::JsonAttribute;
use crate::interface::AmInterface;

use crate::{interface, subscription};
use crate::interface::builder::Builder as InterfaceBuilder;


use crate::Error as PlatformError;

use crate::FunctionResult as PlatformFunctionResult;

pub struct BlcParams {
    pub power_min: f64,
    pub power_max: f64,
    pub power_decimals: i32,

    pub current_min: f64,
    pub current_max: f64,
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

    async fn read_power_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError>;

    async fn write_power_value(&mut self, interface: &AmInterface, v: f64) -> Result<(), PlatformError>;

    async fn read_current_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError>;

    async fn write_current_value(&mut self, interface: &AmInterface, v: f64) -> Result<(), PlatformError>;

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
    blc_interface: Arc<Mutex<BlcInterface>>
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
    async fn initializating(&self, interface: &AmInterface) -> Result<(), PlatformError>
    {
        let mut blc_itf = self.blc_interface.lock().await;

        // Custom initialization slot
        blc_itf.actions.initializating(&interface).await.unwrap();

        // Register attributes
        interface.lock().await.register_attribute(JsonAttribute::new_boxed("mode", true));
        interface.lock().await.register_attribute(JsonAttribute::new_boxed("enable", true));
        interface.lock().await.register_attribute(JsonAttribute::new_boxed("power", true));
        interface.lock().await.register_attribute(JsonAttribute::new_boxed("current", true));

        // Init mode
        let mode_value = blc_itf.actions.read_mode_value(&interface).await.unwrap();
        interface.lock().await.update_attribute_with_string("mode", "value", &mode_value);

        // Init enable
        let enable_value = blc_itf.actions.read_enable_value(&interface).await.unwrap();
        interface.lock().await.update_attribute_with_bool("enable", "value", enable_value).unwrap();

        // Init power
        interface.lock().await.update_attribute_with_f64("power", "min", blc_itf.params.power_min );
        interface.lock().await.update_attribute_with_f64("power", "max", blc_itf.params.power_max );
        interface.lock().await.update_attribute_with_f64("power", "value", 0.0);
        interface.lock().await.update_attribute_with_f64("power", "decimals", blc_itf.params.power_decimals as f64);
        interface.lock().await.update_attribute_with_f64("power", "polling_cycle", 0.0);

        // Init current
        interface.lock().await.update_attribute_with_f64("current", "min", blc_itf.params.current_min );
        interface.lock().await.update_attribute_with_f64("current", "max", blc_itf.params.current_max );
        interface.lock().await.update_attribute_with_f64("current", "value", 0.0);
        interface.lock().await.update_attribute_with_f64("current", "decimals", blc_itf.params.current_decimals as f64);
        interface.lock().await.update_attribute_with_f64("current", "polling_cycle", 0.0);

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

    async fn warning(&self, interface: &AmInterface)
    {
        println!("warning");
    }


    async fn cleaning(&self, _interface: &AmInterface)
    {
        println!("cleaning");
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
    blc_interface: Arc<Mutex<BlcInterface>>
}

impl BlcSubscriber {

    /// 
    /// 
    #[inline(always)]
    async fn process_mode_value(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, field_data: &Value) {
        let requested_value = field_data.as_str().unwrap().to_string();
        let _ =self.blc_interface.lock().await
            .actions.write_mode_value(&interface, requested_value).await;

        let r_value = self.blc_interface.lock().await
            .actions.read_mode_value(&interface).await
            .unwrap();

        interface.lock().await
            .update_attribute_with_string("mode", "value", &r_value);
    }

    /// 
    /// 
    #[inline(always)]
    async fn process_enable_value(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, field_data: &Value) {
        let requested_value = field_data.as_bool().unwrap();
        let _ = self.blc_interface.lock().await
            .actions.write_enable_value(&interface, requested_value).await;

        let r_value = self.blc_interface.lock().await
            .actions.read_enable_value(&interface).await
            .unwrap();

        interface.lock().await
            .update_attribute_with_bool("enable", "value", r_value).unwrap();
    }

    /// 
    /// 
    #[inline(always)]
    async fn process_power_value(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, field_data: &Value) {
        let requested_value = field_data.as_f64().unwrap();
        let _ = self.blc_interface.lock().await
            .actions.write_power_value(&interface, requested_value as f64).await;

        let r_value = self.blc_interface.lock().await
            .actions.read_power_value(&interface).await
            .unwrap();

        interface.lock().await
            .update_attribute_with_f64("power", "value", r_value as f64);
    }

    /// 
    /// 
    #[inline(always)]
    async fn process_current_value(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, field_data: &Value) {
        let requested_value = field_data.as_f64().unwrap();
        let _ = self.blc_interface.lock().await
            .actions.write_current_value(&interface, requested_value as f64).await;

        let r_value = self.blc_interface.lock().await
            .actions.read_current_value(&interface).await
            .unwrap();

        interface.lock().await
            .update_attribute_with_f64("current", "value", r_value as f64);
    }


}

#[async_trait]
impl interface::subscriber::Subscriber for BlcSubscriber {

    /// Get the list of attributes names
    ///
    async fn attributes_names(&self) -> Vec<(subscription::Id, String)> {
        return vec![
            (ID_MODE, "mode".to_string()),
            (ID_ENABLE, "enable".to_string()),
            (ID_POWER, "power".to_string()),
            (ID_CURRENT, "current".to_string())
        ];
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
                    let oo = serde_json::from_slice::<Value>(payload).unwrap();
                    let o = oo.as_object().unwrap();


                    for (attribute_name, fields) in o.iter() {
                        for (field_name, field_data) in fields.as_object().unwrap().iter() {
                            if attribute_name == "mode" && field_name == "value" {
                                self.process_mode_value(&interface, attribute_name, field_name, field_data).await;
                            }
                            else if attribute_name == "enable" && field_name == "value" {
                                self.process_enable_value(&interface, attribute_name, field_name, field_data).await;
                            }
                            else if attribute_name == "power" && field_name == "value" {
                                self.process_power_value(interface, attribute_name, field_name, field_data).await;
                            }
                            else if attribute_name == "current" && field_name == "value" {
                                self.process_current_value(interface, attribute_name, field_name, field_data).await;
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
    actions: Box<dyn BlcActions>
) -> InterfaceBuilder {

    let c = BlcInterface::new_am(params, actions);

    return InterfaceBuilder::new(
        name,
        "blc",
        "0.0",
        Box::new(BlcStates{blc_interface: c.clone()}),
        Box::new(BlcSubscriber{blc_interface: c.clone()})
    );
}

