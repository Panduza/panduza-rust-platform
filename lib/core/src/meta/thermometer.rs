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

pub struct ThermometerParams {
    pub measure_decimals: i32,
}

#[async_trait]
pub trait ThermometerActions: Send + Sync {

    /// Initialize the interface
    /// The connector initialization must be done here
    ///
    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError>;

    async fn read_measure_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError>;

}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------


async fn update_measure(duration_between_measures: u64, interface: AmInterface, powermeter_state: Arc<Mutex<ThermometerInterface>>) {
    let mut interval = time::interval(time::Duration::from_millis(duration_between_measures));
    loop {
        let r_value = powermeter_state.lock().await
            .actions.read_measure_value(&interface).await
            .unwrap();

        interface.lock().await
            .update_attribute_with_f64("measure", "value", r_value as f64);
        
        interface.lock().await.publish_all_attributes().await;

        interval.tick().await;
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

struct ThermometerInterface {

    params: ThermometerParams,
    actions: Box<dyn ThermometerActions>
}
type AmThermometerInterface = Arc<Mutex<ThermometerInterface>>;

impl ThermometerInterface {
    fn new(params: ThermometerParams, actions: Box<dyn ThermometerActions>) -> ThermometerInterface {
        return ThermometerInterface {
            params: params,
            actions: actions
        }
    }
    fn new_am(params: ThermometerParams, actions: Box<dyn ThermometerActions>) -> AmThermometerInterface {
        return Arc::new(Mutex::new( ThermometerInterface::new(params, actions) ));
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

struct ThermometerStates {
    thermometer_interface: Arc<Mutex<ThermometerInterface>>
}


#[async_trait]
impl interface::fsm::States for ThermometerStates {

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
        let mut thermometer_itf = self.thermometer_interface.lock().await;

        // Custom initialization slot
        thermometer_itf.actions.initializating(&interface).await.unwrap();

        // Register attributes
        interface.lock().await.register_attribute(JsonAttribute::new_boxed("measure", true));

        // Init measure
        interface.lock().await.update_attribute_with_f64("measure", "value", 0.0);
        interface.lock().await.update_attribute_with_f64("measure", "decimals", thermometer_itf.params.measure_decimals as f64);
        interface.lock().await.update_attribute_with_f64("measure", "polling_cycle", 0.0);

        // Publish all attributes for start
        interface.lock().await.publish_all_attributes().await;

        let thermometer_interface = Arc::clone(&(self.thermometer_interface));
        let interface_cloned = Arc::clone(&interface);

        tokio::spawn(update_measure(1000, interface_cloned, thermometer_interface));

        // Notify the end of the initialization
        interface.lock().await.set_event_init_done();
    }

    async fn running(&self, interface: &AmInterface)
    {
        println!("running");


        interface::basic::wait_for_fsm_event(interface).await;
    }

    async fn error(&self, _interface: &AmInterface)
    {
        println!("error");
    }

    // async fn cleaning(&self, _interface: &AmInterface)
    // {
    //     println!("cleaning");
    // }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

const ID_MEASURE: subscription::Id = 0;

struct ThermometerSubscriber {
    thermometer_interface: Arc<Mutex<ThermometerInterface>>
}

impl ThermometerSubscriber {

    /// 
    /// 
    #[inline(always)]
    async fn process_measure_value(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, _field_data: &Value) {
        let r_value = self.thermometer_interface.lock().await
            .actions.read_measure_value(&interface).await
            .unwrap();

        interface.lock().await
            .update_attribute_with_f64("measure", "value", r_value as f64);
    }


}

#[async_trait]
impl interface::subscriber::Subscriber for ThermometerSubscriber {

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
                    // interface.lock().await.publish_info().await;

                    // only when running state

                    println!("ThermometerSubscriber::process: {:?}", msg.topic());
                    println!("ThermometerSubscriber::process: {:?}", msg.payload());

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

/// Build the meta interface for a Thermometer Channel
///
pub fn build<A: Into<String>>(
    name: A,
    params: ThermometerParams,
    actions: Box<dyn ThermometerActions>
) -> InterfaceBuilder {

    let c = ThermometerInterface::new_am(params, actions);

    return InterfaceBuilder::new(
        name,
        "thermometer",
        "0.0",
        Box::new(ThermometerStates{thermometer_interface: c.clone()}),
        Box::new(ThermometerSubscriber{thermometer_interface: c.clone()})
    );
}

