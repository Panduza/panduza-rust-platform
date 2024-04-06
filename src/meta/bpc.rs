use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::platform::PlatformError;
use crate::{interface, subscription};
use crate::interface::builder::Builder as InterfaceBuilder;

pub struct BpcParams {
    pub voltage_min: f32,
    pub voltage_max: f32,
}

#[async_trait]
pub trait BpcActions: Send + Sync {

    async fn read_enable_value(&self) -> Result<bool, PlatformError>;

    async fn write_enable_value(&self, v: bool);

    async fn read_voltage_value(&self) -> Result<f32, PlatformError>;

    async fn write_voltage_value(&self, v: f32);


// async def _PZA_DRV_BPC_read_voltage_decimals(self):
//     """Must return the number of decimals supported for the voltage
//     """
//     raise NotImplementedError("Must be implemented !")

// # ---

// async def _PZA_DRV_BPC_read_current_value(self):
//     """Must get the current value value on the BPC and return it
//     """
//     raise NotImplementedError("Must be implemented !")

// async def _PZA_DRV_BPC_write_current_value(self, v):
//     """Must set *v* as the new current value value on the BPC
//     """
//     raise NotImplementedError("Must be implemented !")

// async def _PZA_DRV_BPC_current_value_min_max(self):
//     """Must return the current range of the power supply
//     """
//     return {"min": 0, "max": 0 }

// async def _PZA_DRV_BPC_read_current_decimals(self):
//     """Must return the number of decimals supported for the amperage
//     """
//     raise NotImplementedError("Must be implemented !")

}



struct BpcCore {
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

struct BpcStates {
    bpc_core: Arc<Mutex<BpcCore>>
}


#[async_trait]
impl interface::fsm::States for BpcStates {

    async fn connecting(&self, core: &interface::AmCore)
    {
        println!("connecting");

        let fsm_events_notifier = core.lock().await.get_fsm_events_notifier();
        fsm_events_notifier.notified().await;
    }

    async fn initializating(&self, core: &interface::AmCore)
    {
        println!("initializating");
        
        let mut p = core.lock().await;
        p.set_event_init_done();
    }

    async fn running(&self, core: &interface::AmCore)
    {
        println!("running");
        
        let fsm_events_notifier = core.lock().await.get_fsm_events_notifier();
        fsm_events_notifier.notified().await;
    }

    async fn error(&self, core: &interface::AmCore)
    {
        println!("error");
    }

}

struct BpcSubscriber {
    bpc_core: Arc<Mutex<BpcCore>>
}


#[async_trait]
impl interface::subscriber::Subscriber for BpcSubscriber {

    async fn attributes_names(&self) -> Vec<(subscription::Id, String)> {
        return vec![
            (0, "enable".to_string()),
            (1, "voltage".to_string()),
            (2, "current".to_string())
        ];
    }

    /// Process a message
    ///
    async fn process(&self, data: &interface::core::AmCore, msg: &subscription::Message) {

    }
}




pub fn new(
    bpc_params: BpcParams,
    bpc_actions: Box<dyn BpcActions>
) -> InterfaceBuilder {

    let c = BpcCore::new_am(bpc_params, bpc_actions);

    return InterfaceBuilder::new(
        "channel",
        "bpc",
        "0.0",
        Box::new(BpcStates{bpc_core: c.clone()}),
        Box::new(BpcSubscriber{bpc_core: c.clone()})
    );
}

