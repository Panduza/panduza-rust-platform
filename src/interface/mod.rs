pub mod fsm;
pub mod basic;
mod runner;
pub mod builder;
pub mod listener;
pub mod subscriber;


use crate::platform::services::AmServices;

use std::collections::HashMap;
use std::sync::Arc;

use rumqttc::AsyncClient;

use tokio::sync::Mutex;
use tokio::sync::Notify;


use crate::interface::fsm::State;
use crate::interface::fsm::Events;

use crate::attribute::InfoAttribute;
use crate::attribute::AttributeInterface;


pub type Builder = builder::Builder;
pub type Runner = runner::Runner;
pub type AmRunner = runner::AmRunner;


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
    client: AsyncClient,

    // -- ATTRIBUTES --
    attributes: HashMap<String, Box<dyn AttributeInterface>>,

    //
    platform_services: AmServices

}
pub type AmInterface = Arc<Mutex<Interface>>;

impl Interface {

    /// Create a new instance of the Core
    ///
    fn new<A: Into<String>, B: Into<String>, C: Into<String>, D: Into<String>, E: Into<String>>
        (name: A, dev_name: B, bench_name: C, itype: D, version: E,
            client: AsyncClient, platform_services: AmServices
        )
        -> Interface {
        let mut obj = Interface {
            name: name.into(),
            dev_name: dev_name.into(),
            bench_name: bench_name.into(),
            topic_base: String::new(),
            topic_cmds: String::new(),
            topic_atts: String::new(),
            topic_info: String::new(),
            client: client,
            fsm_state: State::Connecting,
            fsm_events: Events::NO_EVENT,
            fsm_events_notifier: Arc::new(Notify::new()),
            attributes: HashMap::new(),
            platform_services: platform_services
        };
        obj.register_attribute(InfoAttribute::new_boxed(itype, version));
        obj.update_topics();
        return obj;
    }

    /// Create a new instance of the Core
    /// 
    pub fn new_am<A: Into<String>, B: Into<String>, C: Into<String>, D: Into<String>, E: Into<String>>
        (name: A, dev_name: B, bench_name: C, itype: D, version: E, client: AsyncClient, platform_services: AmServices)
            -> AmInterface
    {
        return Arc::new(Mutex::new(
            Interface::new(name, dev_name, bench_name, itype, version, client, platform_services)
        ));
    }

    // -- IDENTITY --

    /// Get the name of the interface
    /// 
    pub fn name(&self) -> &String {
        return &self.name;
    }

    /// Get the name of the device
    /// 
    pub fn dev_name(&self) -> &String {
        return &self.dev_name;
    }

    /// Get the name of the bench
    /// 
    pub fn bench_name(&self) -> &String {
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

    /// Clear the events
    ///
    pub fn clear_events(&mut self) {
        self.fsm_events = Events::NO_EVENT;
    }

    /// Move to a new state
    ///
    pub fn move_to_state(&mut self, state: State) {
        let previous = self.fsm_state.clone();
        self.fsm_state = state;
        self.update_attribute_with_string("info", "state", &self.fsm_state.to_string());

        self.log_info(format!("State changed {:?} => {:?}", previous, self.fsm_state));
    }

    /// Get the fsm events notifier
    ///
    pub fn get_fsm_events_notifier(&self) -> Arc<Notify> {
        return self.fsm_events_notifier.clone();
    }
    // -- Event Setters --
    pub fn set_event_connection_up(&mut self) {
        self.fsm_events.insert(Events::CONNECTION_UP);
        self.fsm_events_notifier.notify_one();
    }
    pub fn set_event_connection_down(&mut self) {
        self.fsm_events.insert(Events::CONNECTION_DOWN);
        self.fsm_events_notifier.notify_one();
    }
    pub fn set_event_init_done(&mut self) {
        self.fsm_events.insert(Events::INIT_DONE);
        self.fsm_events_notifier.notify_one();
    }
    // pub fn set_event_state_error(&mut self) {
    //     self.fsm_events.insert(Events::ERROR);
    //     self.fsm_events_notifier.notify_one();
    // }

    // -- CLIENT --

    /// Get the base topic
    ///
    pub async fn publish(&self, topic: &str, payload: &str, retain: bool) {
        self.client.publish(topic, rumqttc::QoS::AtLeastOnce, retain, payload).await.unwrap();
    }

    // -- ATTRIBUTES --

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

    pub async fn publish_all_attributes(&self) {
        for (_, attribute) in self.attributes.iter() {
            self.publish_attribute(attribute.name()).await;
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
    pub fn update_attribute_with_bool(&mut self, attribute: &str, field: &str, value: bool) {
        let att = self.attributes.get_mut(attribute).unwrap();
        att.as_mut().update_field_with_bool(field, value);
    }
    pub fn update_attribute_with_string(&mut self, attribute: &str, field: &str, value: &String) {
        let att = self.attributes.get_mut(attribute).unwrap();
        att.as_mut().update_field_with_string(field, value);
    }
    pub fn update_attribute_with_json(&mut self, attribute: &str, field: &str, value: &serde_json::Value) {
        let att = self.attributes.get_mut(attribute).unwrap();
        att.as_mut().update_field_with_json(field, value);
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

}
