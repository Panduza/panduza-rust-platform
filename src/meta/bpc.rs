use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::attribute::JsonAttribute;
use crate::interface::AmInterface;
use crate::platform::PlatformError;
use crate::{interface, subscription};
use crate::interface::builder::Builder as InterfaceBuilder;

pub struct BpcParams {
    pub voltage_min: f32,
    pub voltage_max: f32,
}

#[async_trait]
pub trait BpcActions: Send + Sync {

    /// Initialize the interface
    /// The connector initialization must be done here
    ///
    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError>;

    async fn read_enable_value(&mut self, interface: &AmInterface) -> Result<bool, PlatformError>;

    async fn write_enable_value(&mut self, interface: &AmInterface, v: bool);

    async fn read_voltage_value(&mut self, interface: &AmInterface) -> Result<f32, PlatformError>;

    async fn write_voltage_value(&mut self, interface: &AmInterface, v: f32);

    async fn read_current_value(&mut self, interface: &AmInterface) -> Result<f32, PlatformError>;

    async fn write_current_value(&mut self, interface: &AmInterface, v: f32);


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


pub struct EnableAttribute {
    attr: JsonAttribute,
}

pub struct F32ValueAttribute {
    attr: JsonAttribute,
}


// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

struct BpcInterface {

    bpc_params: BpcParams,
    bpc_actions: Box<dyn BpcActions>
}
type AmBpcInterface = Arc<Mutex<BpcInterface>>;

impl BpcInterface {
    fn new(bpc_params: BpcParams, bpc_actions: Box<dyn BpcActions>) -> BpcInterface {
        return BpcInterface {
            bpc_params: bpc_params,
            bpc_actions: bpc_actions
        }
    }
    fn new_am(bpc_params: BpcParams, bpc_actions: Box<dyn BpcActions>) -> AmBpcInterface {
        return Arc::new(Mutex::new( BpcInterface::new(bpc_params, bpc_actions) ));
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

struct BpcStates {
    bpc_interface: Arc<Mutex<BpcInterface>>
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
    {
        // Custom initialization slot
        self.bpc_interface.lock().await.bpc_actions.initializating(&interface).await.unwrap();

        // Register attributes
        interface.lock().await.register_attribute(JsonAttribute::new_boxed("enable", true));
        interface.lock().await.register_attribute(JsonAttribute::new_boxed("voltage", true));
        interface.lock().await.register_attribute(JsonAttribute::new_boxed("current", true));

        // Init enable
        let enable_value = self.bpc_interface.lock().await.bpc_actions.read_enable_value(&interface).await.unwrap();
        interface.lock().await.update_attribute_with_bool("enable", "value", enable_value);

        // Init voltage
        interface.lock().await.update_attribute_with_f32("voltage", "min", 0.0);
        interface.lock().await.update_attribute_with_f32("voltage", "max", 0.0);
        interface.lock().await.update_attribute_with_f32("voltage", "value", 0.0);
        interface.lock().await.update_attribute_with_f32("voltage", "decimals", 0.0);
        interface.lock().await.update_attribute_with_f32("voltage", "polling_cycle", 0.0);

        // Init current
        interface.lock().await.update_attribute_with_f32("current", "min", 0.0);
        interface.lock().await.update_attribute_with_f32("current", "max", 0.0);
        interface.lock().await.update_attribute_with_f32("current", "value", 0.0);
        interface.lock().await.update_attribute_with_f32("current", "decimals", 0.0);
        interface.lock().await.update_attribute_with_f32("current", "polling_cycle", 0.0);

        // Publish all attributes for start
        interface.lock().await.publish_all_attributes().await;

        // Notify the end of the initialization
        interface.lock().await.set_event_init_done();
    }

    async fn running(&self, interface: &AmInterface)
    {
        println!("running");
        
        let fsm_events_notifier = interface.lock().await.get_fsm_events_notifier();
        fsm_events_notifier.notified().await;
    }

    async fn error(&self, interface: &AmInterface)
    {
        println!("error");
    }

    async fn cleaning(&self, interface: &AmInterface)
    {
        println!("cleaning");
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
    bpc_interface: Arc<Mutex<BpcInterface>>
}

#[async_trait]
impl interface::subscriber::Subscriber for BpcSubscriber {

    /// Get the list of attributes names
    ///
    async fn attributes_names(&self) -> Vec<(subscription::Id, String)> {
        return vec![
            (ID_ENABLE, "enable".to_string()),
            (ID_VOLTAGE, "voltage".to_string()),
            (ID_CURRENT, "current".to_string())
        ];
    }

    /// Process a message
    ///
    async fn process(&self, interface: &AmInterface, msg: &subscription::Message) {
        // Common processing
        interface::basic::process(&interface, msg).await;

        match msg {
            subscription::Message::Mqtt(msg) => {
                match msg.id() {
                    subscription::ID_PZA_CMDS_SET => {
                        // interface.lock().await.publish_info().await;

                        println!("BpcSubscriber::process: {:?}", msg.topic());
                        println!("BpcSubscriber::process: {:?}", msg.payload());


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
    bpc_params: BpcParams,
    bpc_actions: Box<dyn BpcActions>
) -> InterfaceBuilder {

    let c = BpcInterface::new_am(bpc_params, bpc_actions);

    return InterfaceBuilder::new(
        name,
        "bpc",
        "0.0",
        Box::new(BpcStates{bpc_interface: c.clone()}),
        Box::new(BpcSubscriber{bpc_interface: c.clone()})
    );
}

