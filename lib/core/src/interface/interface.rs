use crate::attribute::Attribute;
use crate::attribute::MqttPayload;
use crate::platform::services::AmServices;
use crate::FunctionResult as PlatformFunctionResult;
// use crate::platform::PlatformError;
// use crate::__platform_error_result;
use crate::__platform_error;

use std::collections::HashMap;
use std::sync::Arc;

use rumqttc::AsyncClient;

use tokio::sync::Mutex;
use tokio::sync::Notify;


use crate::interface::fsm::State;
use crate::interface::fsm::Events;

use crate::attribute::InfoAttribute;
use crate::attribute::AttributeInterface;
use crate::attribute::ThreadSafeAttribute;

use super::logger::Logger;
use super::ThreadSafeInterface;





/// Shared data and behaviour across an interface objects
/// 
pub struct Interface {

    // -- IDENTITY --
    /// Name of the interface
    name: String,
    /// Name of the device
    dev_name: String,
    /// Name of the bench
    bench_name: String,

    // Topics
    topic_base: String,
    topic_cmds: String,
    topic_atts: String,
    topic_info: String,

    // -- FSM --
    /// Current state
    fsm_state: State,

    /// Events
    fsm_events: Events,
    /// Notifier for events
    fsm_events_notifier: Arc<Notify>,

    // -- CLIENT --

    is_connection_up: bool,

    client: AsyncClient,

    // -- ATTRIBUTES --
    attributes: HashMap<String, Box<dyn AttributeInterface>>,

    map_of_attributes: HashMap<String, ThreadSafeAttribute>,

    //
    pub platform_services: AmServices,



    
    last_error_message: Option<String>,


    // -- LOGS --
    logger: Logger
}
pub type AmInterface = Arc<Mutex<Interface>>;

impl Interface {

    /// Create a new instance of the Core
    ///
    pub fn new<A: Into<String>, B: Into<String>, C: Into<String>, D: Into<String>, E: Into<String>>
        (name: A, dev_name: B, bench_name: C, itype: D, version: E,
            client: AsyncClient, platform_services: AmServices
        )
        -> Interface {

        let string_name = name.into();
        let string_dev_name = dev_name.into();
        let string_bench_name = bench_name.into();

        let mut obj = Interface {
            name: string_name.clone(),
            dev_name: string_dev_name.clone(),
            bench_name: string_bench_name.clone(),
            topic_base: String::new(),
            topic_cmds: String::new(),
            topic_atts: String::new(),
            topic_info: String::new(),
            is_connection_up: false,
            client: client,
            fsm_state: State::Connecting,
            fsm_events: Events::NO_EVENT,
            fsm_events_notifier: Arc::new(Notify::new()),
            attributes: HashMap::new(),
            map_of_attributes: HashMap::new(),
            platform_services: platform_services,
            last_error_message: None,
            logger: Logger::new(string_bench_name.clone(), string_dev_name.clone(), string_name.clone())
        };
        obj.register_attribute(InfoAttribute::new_boxed(itype, version));
        obj.update_topics();
        return obj;
    }

    /// Wrap the interface in a thread safe container
    /// 
    pub fn as_thread_safe(self: Self) -> ThreadSafeInterface {
        return Arc::new(Mutex::new(self));
    }

    // -- IDENTITY --

    /// Get the name of the interface
    /// 
    pub fn name(&self) -> &String {
        return &self.name;
    }

    /// Get the name of the device
    /// 
    pub fn _dev_name(&self) -> &String {
        return &self.dev_name;
    }

    /// Get the name of the bench
    /// 
    pub fn _bench_name(&self) -> &String {
        return &self.bench_name;
    }

    /// Update topics after a name change
    fn update_topics(&mut self) {
        self.topic_base = format!("pza/{}/{}/{}", self.bench_name, self.dev_name, self.name);
        self.topic_cmds = format!("{}/cmds", self.topic_base);
        self.topic_atts = format!("{}/atts", self.topic_base);
        self.topic_info = format!("{}/info", self.topic_atts);
    }


    // -- FSM --

    /// Get the current state
    ///
    pub fn current_state(&self) -> &State {
        return &self.fsm_state;
    }

    /// Get the events
    ///
    pub fn events(&mut self) -> &mut Events {
        return &mut self.fsm_events;
    }

    /// Move to a new state
    ///
    pub fn move_to_state(&mut self, state: State) {
        let previous = self.fsm_state.clone();
        self.fsm_state = state;
        self.update_attribute_with_string("info", "state", &self.fsm_state.to_string());

        let p = self.last_error_message.clone().or(Some("".to_string())).unwrap().clone();
        self.update_attribute_with_string("info", "error",  &p);

        self.log_info(format!("State changed {:?} => {:?}", previous, self.fsm_state));
    }

    // ------------------------------------------------------------------------
    // -- EVENTS --

    /// Get the fsm events notifier
    ///
    pub fn get_fsm_events_notifier(&self) -> Arc<Notify> {
        return self.fsm_events_notifier.clone();
    }

    /// Clear the events
    ///
    pub fn clear_events(&mut self) {
        self.fsm_events = Events::NO_EVENT;
    }
    
    /// Generic event triggerer
    /// 
    pub fn set_event(&mut self, event: Events) {
        self.fsm_events.insert(event);
        self.fsm_events_notifier.notify_one();
    }

    /// Trigger an error event
    /// Error event must also provide an error message
    /// 
    pub fn set_event_error<A: Into<String>>(&mut self, message: A) {
        self.last_error_message = Some(message.into());
        self.fsm_events.insert(Events::ERROR);
        self.fsm_events_notifier.notify_one();
    }

    // Helpers for event trigger
    #[inline(always)]
    pub fn set_event_connection_up(&mut self) { 
        self.is_connection_up = true; // bidouille pas propre
        self.set_event(Events::CONNECTION_UP);
    }
    #[inline(always)]
    pub fn set_event_connection_down(&mut self) {
        self.is_connection_up = false; // bidouille pas propre
        self.set_event(Events::CONNECTION_DOWN);
    }
    #[inline(always)]
    pub fn set_event_init_done(&mut self) { self.set_event(Events::INIT_DONE); }
    #[inline(always)]
    pub fn set_event_reboot(&mut self) { self.set_event(Events::REBOOT); }
    #[inline(always)]
    pub fn set_event_cleaned(&mut self) { self.set_event(Events::CLEANED); }


    pub fn trigger_event_connection_cache(&mut self) {
        if self.is_connection_up {
            self.set_event_connection_up();
        }
    }

    // -- CLIENT --

    /// Get the base topic
    ///
    pub async fn publish(&self, topic: &str, payload: &MqttPayload, retain: bool) {
        self.log_debug(
            format!("publish attribute {:?} {:?}", topic, retain)
        );

        // HERE the rumqtt copy the payload inside its own buffer
        // It is ok for small payloads but not for big ones, there will be work here for streams

        let publish_result = self.client.publish(topic, rumqttc::QoS::AtLeastOnce, retain, payload).await;

        match publish_result {
            Ok(_) => {},
            Err(e) => {
                println!("Failed to publish payload : {}", e);
            }
        }

    }

    // -- ATTRIBUTES --



    pub async fn add_attribute(&mut self, attribute: ThreadSafeAttribute) {

        let name = attribute.lock().await.name().clone();

        self.log_debug(
            format!("Create attribute {:?}", name)
        );

        self.map_of_attributes.insert(name, attribute);

    }



    pub fn create_attribute(&mut self, attribute: Attribute) -> ThreadSafeAttribute {
        self.log_debug(
            format!("Create attribute {:?}", attribute.name())
        );

        let name = attribute.name().clone();
        let ts_att = crate::attribute::pack_attribute_as_thread_safe(attribute);

        self.map_of_attributes.insert(name, ts_att.clone());

        ts_att.clone()
    }



    /// Register a new attribute
    /// 
    pub fn register_attribute(&mut self, attribute: Box<dyn AttributeInterface>) {
        self.log_debug(
            format!("Registering attribute {:?}", attribute.name())
        );
        self.attributes.insert(attribute.name().clone(), attribute);
    }

    /// Get an attribute
    /// 
    pub fn attribute(&self, name: &str) -> Option<&Box<dyn AttributeInterface>> {
        return self.attributes.get(name);
    }

    /// Publish an attribute
    ///
    pub async fn publish_attribute(&self, name: &str) {
        let attribute = self.attribute(name).unwrap();
        self.publish(
            format!("{}/{}", self.topic_atts, attribute.name()).as_str()
            , &attribute.to_mqtt_payload(), attribute.retain().clone()).await;
    }

    pub async fn publish_attribute_bis(&self, name: &str) {
        let attribute = 
            self.map_of_attributes.get(name).unwrap().lock().await;
        self.publish(
            format!("{}/{}", self.topic_atts, name).as_str()
            , &MqttPayload::Bytes(attribute.to_vec().clone()), attribute.retain()).await;
    }

    pub async fn publish_all_attributes(&self) {
        for (_, attribute) in self.attributes.iter() {
            self.publish_attribute(attribute.name()).await;
        }

        for (_, attribute) in self.map_of_attributes.iter() {
            println!("---------------------");
            let namme = attribute.lock().await.name();            
            println!("pok {:?}", namme);
            self.publish_attribute_bis(namme.as_str()).await;
            println!("--------------------- end ");
        }

    }

    /// Publish the info
    ///
    pub async fn publish_info(&self) {
        self.publish_attribute("info").await;
    }



    pub fn update_attribute_with_f64(&mut self, attribute: &str, field: &str, value: f64) {
        let att = self.attributes.get_mut(attribute).unwrap();
        att.as_mut().update_field_with_f64(field, value);
    }

    /// Update an attribute with a boolean value
    ///
    pub fn update_attribute_with_bool(&mut self, attribute: &str, field: &str, value: bool) -> PlatformFunctionResult {
        // Trace
        self.logger.log_trace(format!("update_attribute_with_bool(att={}, field={}, value={})", attribute, field, value));
        // Action
        self.attributes.get_mut(attribute)
            .ok_or(
                __platform_error!(format!("Attribute ({:?}) not found", attribute))
            )
            .and_then(|att| {
                att.as_mut().update_field_with_bool(field, value)
            })
    }

    pub fn update_attribute_with_string(&mut self, attribute: &str, field: &str, value: &String) {
        let att = self.attributes.get_mut(attribute).unwrap();
        att.as_mut().update_field_with_string(field, value);
    }

    pub fn update_attribute_with_json(&mut self, attribute: &str, field: &str, value: &serde_json::Value) {
        let att = self.attributes.get_mut(attribute).unwrap();
        att.as_mut().update_field_with_json(field, value);
    }

    pub fn update_attribute_with_bytes(&mut self, attribute: &str, value: &Vec<u8>) {
        let att = self.attributes.get_mut(attribute).unwrap();
        att.as_mut().push_byte_stream(value);
    }

    pub fn platform_services(&self) -> AmServices {
        return self.platform_services.clone();
    }


    // -- LOGS --

    /// Log trace
    ///
    #[inline]
    pub fn log_warn<A: Into<String>>(&self, text: A) {
        tracing::warn!(class="Interface", bname=self.bench_name, dname=self.dev_name, iname=self.name, 
            "{}", text.into());
    }

    /// Log info
    ///
    #[inline]
    pub fn log_info<A: Into<String>>(&self, text: A) {
        tracing::info!(class="Interface", bname=self.bench_name, dname=self.dev_name, iname=self.name, 
            "{}", text.into());
    }

    /// Log debug
    ///
    #[inline]
    pub fn log_debug<A: Into<String>>(&self, text: A) {
        tracing::debug!(class="Interface", bname=self.bench_name, dname=self.dev_name, iname=self.name, 
            "{}", text.into());
    }

    // /// Log trace
    // ///
    // #[inline]
    // pub fn log_trace<A: Into<String>>(&self, text: A) {
    //     tracing::trace!(class="Interface", bname=self.bench_name, dname=self.dev_name, iname=self.name, 
    //         "{}", text.into());
    // }


    /// Clone the logger of the interface
    /// 
    pub fn clone_logger(&self) -> Logger {
        return self.logger.clone();
    }

    /// Log the last error message
    ///
    pub fn log_last_error(&self) {
        match &self.last_error_message {
            Some(msg) => {
                self.logger.log_warn(format!("Last error message: {}", msg));
            },
            None => {
                self.logger.log_warn("No error message");
            }
        }
    }

}
