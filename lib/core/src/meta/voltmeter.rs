use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::Mutex;
use tokio::time;

use crate::attribute::JsonAttribute;
use crate::interface::AmInterface;

use crate::{interface, subscription};
use crate::interface::builder::Builder as InterfaceBuilder;

use crate::Error as PlatformError;

use crate::FunctionResult as PlatformFunctionResult;

pub struct VoltmeterParams {
    pub measure_decimals: i32,
}

#[async_trait]
pub trait VoltmeterActions: Send + Sync {
    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError>;
    async fn read_measure_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError>;
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

struct VoltmeterInterface {
    params: VoltmeterParams,
    actions: Box<dyn VoltmeterActions>
}

type AmVoltmeterInterface = Arc<Mutex<VoltmeterInterface>>;

impl VoltmeterInterface {
    fn new(params: VoltmeterParams, actions: Box<dyn VoltmeterActions>) -> VoltmeterInterface {
        return VoltmeterInterface {
            params: params,
            actions: actions
        }
    }
    fn new_am(params: VoltmeterParams, actions: Box<dyn VoltmeterActions>) -> AmVoltmeterInterface {
        return Arc::new(Mutex::new( VoltmeterInterface::new(params, actions) ));
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

struct VoltmeterStates {
    voltmeter_interface: Arc<Mutex<VoltmeterInterface>>
}


#[async_trait]
impl interface::fsm::States for VoltmeterStates {

    /// Just wait for an fsm event for the connection
    ///
    async fn connecting(&self, interface: &AmInterface)
    {
        interface::basic::wait_for_fsm_event(interface).await;
    }

    async fn warning(&self, _interface: &AmInterface)
    {
        println!("cleaning");
    }

    async fn cleaning(&self, _interface: &AmInterface)
    {
        println!("cleaning");
    }

    /// Initialize the interface
    ///
    async fn initializating(&self, interface: &AmInterface) -> Result<(), PlatformError>
    {
        let mut voltmeter_itf = self.voltmeter_interface.lock().await;

        // Custom initialization slot
        voltmeter_itf.actions.initializating(&interface).await.unwrap();

        // Register attributes
        interface.lock().await.register_attribute(JsonAttribute::new_boxed("measure", true));

        // Init measure
        interface.lock().await.update_attribute_with_f64("measure", "value", 0.0);
        interface.lock().await.update_attribute_with_f64("measure", "decimals", voltmeter_itf.params.measure_decimals as f64);
        interface.lock().await.update_attribute_with_f64("measure", "polling_cycle", 0.0);

        // Publish all attributes for start
        interface.lock().await.publish_all_attributes().await;
        
        let voltmeter_interface = Arc::clone(&(self.voltmeter_interface));
        let interface_cloned = Arc::clone(&interface);

        // Notify the end of the initialization
        interface.lock().await.set_event_init_done();

        Ok(())
    }

    async fn running(&self, interface: &AmInterface)
    {
        println!("running");


        interface::basic::wait_for_fsm_event(interface).await;
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

const ID_MEASURE: subscription::Id = 0;

struct VoltmeterSubscriber {
    voltmeter_interface: Arc<Mutex<VoltmeterInterface>>
}

impl VoltmeterSubscriber {

    /// 
    /// 
    #[inline(always)]
    async fn process_measure_value(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, _field_data: &Value) {
        let r_value = self.voltmeter_interface.lock().await
            .actions.read_measure_value(&interface).await
            .unwrap();

        interface.lock().await
            .update_attribute_with_f64("measure", "value", r_value as f64);
    }


}

#[async_trait]
impl interface::subscriber::Subscriber for VoltmeterSubscriber {

    /// Get the list of attributes names
    ///
    async fn attributes_names(&self) -> Vec<(subscription::Id, String)> {
        return vec![
            (ID_MEASURE, "measure".to_string())
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

                    // only when running state

                    println!("VoltmeterSubscriber::process: {:?}", msg.topic());
                    println!("VoltmeterSubscriber::process: {:?}", msg.payload());

                    let payload = msg.payload();
                    let oo = serde_json::from_slice::<Value>(payload).unwrap();
                    let o = oo.as_object().unwrap();


                    for (attribute_name, fields) in o.iter() {
                        for (field_name, field_data) in fields.as_object().unwrap().iter() {
                            if attribute_name == "measure" && field_name == "value" {
                                self.process_measure_value(&interface, attribute_name, field_name, field_data).await;
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

/// Build the meta interface for a Voltmeter Channel
///
pub fn build<A: Into<String>>(
    name: A,
    params: VoltmeterParams,
    actions: Box<dyn VoltmeterActions>
) -> InterfaceBuilder {

    let c = VoltmeterInterface::new_am(params, actions);

    return InterfaceBuilder::new(
        name,
        "voltmeter",
        "0.0",
        Box::new(VoltmeterStates{voltmeter_interface: c.clone()}),
        Box::new(VoltmeterSubscriber{voltmeter_interface: c.clone()})
    );
}
