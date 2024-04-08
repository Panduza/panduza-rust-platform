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

struct BpcCore {

    // enable: EnableAttribute,
    // voltage: F32ValueAttribute,
    // current: F32ValueAttribute,

    bpc_params: BpcParams,
    bpc_actions: Box<dyn BpcActions>
}
type AmBpcCore = Arc<Mutex<BpcCore>>;

impl BpcCore {
    fn new(bpc_params: BpcParams, bpc_actions: Box<dyn BpcActions>) -> BpcCore {
        return BpcCore {
            bpc_params: bpc_params,
            bpc_actions: bpc_actions
        }
    }
    fn new_am(bpc_params: BpcParams, bpc_actions: Box<dyn BpcActions>) -> AmBpcCore {
        return Arc::new(Mutex::new( BpcCore::new(bpc_params, bpc_actions) ));
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

struct BpcStates {
    bpc_core: Arc<Mutex<BpcCore>>
}


#[async_trait]
impl interface::fsm::States for BpcStates {

    async fn connecting(&self, core: &AmInterface)
    {
        let fsm_events_notifier = core.lock().await.get_fsm_events_notifier();
        fsm_events_notifier.notified().await;
    }

    async fn initializating(&self, core: &AmInterface)
    {
        self.bpc_core.lock().await.bpc_actions.initializating(&core).await.unwrap();

        let mut p = core.lock().await;
        p.set_event_init_done();
    }

    async fn running(&self, core: &AmInterface)
    {
        println!("running");
        
        let fsm_events_notifier = core.lock().await.get_fsm_events_notifier();
        fsm_events_notifier.notified().await;
    }

    async fn error(&self, core: &AmInterface)
    {
        println!("error");
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
    bpc_core: Arc<Mutex<BpcCore>>
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
    async fn process(&self, core: &AmInterface, msg: &subscription::Message) {
        // Common processing
        interface::basic::process(core, msg).await;

        match msg {
            subscription::Message::Mqtt(msg) => {
                match msg.id() {
                    subscription::ID_PZA_CMDS_SET => {
                        // core.lock().await.publish_info().await;

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

    let c = BpcCore::new_am(bpc_params, bpc_actions);

    return InterfaceBuilder::new(
        name,
        "bpc",
        "0.0",
        Box::new(BpcStates{bpc_core: c.clone()}),
        Box::new(BpcSubscriber{bpc_core: c.clone()})
    );
}

