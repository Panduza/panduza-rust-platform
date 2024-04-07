use rumqttc::tokio_rustls::rustls::crypto::hash::Hash;
use serde_json;

use std::collections::HashMap;
use std::sync::Arc;

use rumqttc::AsyncClient;

use tokio::sync::Mutex;
use tokio::sync::Notify;

use crate::interface::fsm::State;
use crate::interface::fsm::Events;

use crate::attribute::InfoAttribute;
use crate::attribute::AttributeInterface;

/// Shared data and behaviour across an interface objects
/// 
pub struct Core {

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

    // -- ATTRIBUTE --
    /// Interface Indentity Info
    info: InfoAttribute,


    attributes: HashMap<String, Box<dyn AttributeInterface>>,
}
pub type AmCore = Arc<Mutex<Core>>;

impl Core {

    /// Create a new instance of the Core
    ///
    fn new<A: Into<String>, B: Into<String>, C: Into<String>, D: Into<String>, E: Into<String>>
        (name: A, dev_name: B, bench_name: C, itype: D, version: E,
            client: AsyncClient
        )
        -> Core {
        let mut obj = Core {
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
            info: InfoAttribute::new(itype, version),
            attributes: HashMap::new(),
        };
        obj.update_topics();
        return obj;
    }

    /// Create a new instance of the Core
    /// 
    pub fn new_am<A: Into<String>, B: Into<String>, C: Into<String>, D: Into<String>, E: Into<String>>
        (name: A, dev_name: B, bench_name: C, itype: D, version: E, client: AsyncClient)
            -> AmCore
    {
        return Arc::new(Mutex::new(
            Core::new(name, dev_name, bench_name, itype, version, client)
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
        self.info.change_state(self.fsm_state.to_string());
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
    pub fn set_event_state_error(&mut self) {
        self.fsm_events.insert(Events::ERROR);
        self.fsm_events_notifier.notify_one();
    }

    // -- CLIENT --

    /// Get the base topic
    ///
    pub async fn publish(&self, topic: &str, payload: &str, retain: bool) {
        self.client.publish(topic, rumqttc::QoS::AtLeastOnce, retain, payload).await.unwrap();
    }

    /// Publish an attribute
    ///
    pub async fn publish_attribute(&self, attribute: &dyn AttributeInterface) {
        self.publish(
            format!("{}/{}", self.topic_atts, attribute.name()).as_str()
            , &attribute.to_mqtt_payload(), attribute.retain().clone()).await;
    }

    /// Publish the info
    ///
    pub async fn publish_info(&self) {
        self.publish_attribute(&self.info).await;
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

    /// Log trace
    ///
    #[inline]
    pub fn log_trace<A: Into<String>>(&self, text: A) {
        tracing::trace!(class="Interface", bname=self.bench_name, dname=self.dev_name, iname=self.name, 
            "{}", text.into());
    }

}
