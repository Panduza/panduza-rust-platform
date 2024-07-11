use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::Mutex;

use crate::attribute::JsonAttribute;
use crate::interface::AmInterface;
use crate::{interface, subscription};
use crate::interface::builder::Builder as InterfaceBuilder;

use crate::Error as PlatformError;
use crate::__platform_error_result;

use crate::FunctionResult as PlatformFunctionResult;

#[async_trait]
pub trait RelayActions: Send + Sync {

    /// Initialize the interface
    /// The connector initialization must be done here
    ///
    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError>;

    // async fn config(&mut self, interface: &AmInterface) -> Result<(), PlatformError>;

    async fn read_state_open(&mut self, interface: &AmInterface) -> Result<bool, PlatformError>;

    async fn write_state_open(&mut self, interface: &AmInterface, v: bool);
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------


// pub struct StateAttribute {
//     attr: JsonAttribute,
// }


// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

struct RelayInterface {

    actions: Box<dyn RelayActions>
}
type AmRelayInterface = Arc<Mutex<RelayInterface>>;

impl RelayInterface {
    fn new(actions: Box<dyn RelayActions>) -> RelayInterface {
        return RelayInterface {
            actions: actions
        }
    }
    fn new_am(actions: Box<dyn RelayActions>) -> AmRelayInterface {
        return Arc::new(Mutex::new( RelayInterface::new(actions) ));
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

struct RelayStates {
    relay_interface: Arc<Mutex<RelayInterface>>
}


#[async_trait]
impl interface::fsm::States for RelayStates {

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
        // Custom initialization slot
        self.relay_interface.lock().await.actions.initializating(&interface).await?;

        // Register attributes
        interface.lock().await.register_attribute(JsonAttribute::new_boxed("state", true));

        // Init state
        let state_value = self.relay_interface.lock().await.actions.read_state_open(&interface).await?;

        interface.lock().await.update_attribute_with_bool("state", "open", state_value)?;

        // Publish all attributes for start
        interface.lock().await.publish_all_attributes().await;

        // Notify the end of the initialization
        interface.lock().await.set_event_init_done();

        Ok(())
    }

    async fn running(&self, interface: &AmInterface)
    {
        // println!("running");


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
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

const ID_STATE: subscription::Id = 0;

struct RelaySubscriber {
    relay_interface: Arc<Mutex<RelayInterface>>
}

impl RelaySubscriber {

    /// 
    /// 
    #[inline(always)]
    async fn process_state_open(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, field_data: &Value)
        -> PlatformFunctionResult
    {
        let requested_value = match field_data.as_bool() {
            Some(bool) => bool,
            None => return __platform_error_result!("State open not provided")
        };
        self.relay_interface.lock().await
            .actions.write_state_open(&interface, requested_value).await;

        let r_value = self.relay_interface.lock().await
            .actions.read_state_open(&interface).await?;

        interface.lock().await
            .update_attribute_with_bool("state", "open", r_value)

    }
}

#[async_trait]
impl interface::subscriber::Subscriber for RelaySubscriber {

    /// Get the list of attributes names
    ///
    async fn attributes_names(&self) -> Vec<(subscription::Id, String)> {
        return vec![
            (ID_STATE, "state".to_string())
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

                    println!("RelaySubscriber::process: {:?}", msg.topic());
                    println!("RelaySubscriber::process: {:?}", msg.payload());

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
                            if attribute_name == "state" && field_name == "open" {
                                self.process_state_open(&interface, attribute_name, field_name, field_data).await?;

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

/// Build the meta interface for a Voxpower Channel
///
pub fn build<A: Into<String>>(
    name: A,
    actions: Box<dyn RelayActions>
) -> InterfaceBuilder {

    let c = RelayInterface::new_am(actions);

    return InterfaceBuilder::new(
        name,
        "relay",
        "0.0",
        Box::new(RelayStates{relay_interface: c.clone()}),
        Box::new(RelaySubscriber{relay_interface: c.clone()})
    );
}

