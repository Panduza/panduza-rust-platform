use rumqttc::AsyncClient;
use serde_json;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::interface::fsm::State;
use crate::interface::fsm::Events;

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

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
    /// Interface Indentity Info
    info: serde_json::Value,

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

    // -- CLIENTS --
    
    // use both
    // operational only

    default_client: Option<AsyncClient>,
    operational_client: Option<AsyncClient>,
}
pub type AmCore = Arc<Mutex<Core>>;

impl Core {

    /// Create a new instance of the Core
    /// 
    pub fn new<A: Into<String>, B: Into<String>, C: Into<String>>
        (name: A, dev_name: B, bench_name: C) -> Core {
        let mut obj = Core {
            name: name.into(),
            dev_name: dev_name.into(),
            bench_name: bench_name.into(),
            topic_base: String::new(),
            topic_cmds: String::new(),
            topic_atts: String::new(),
            topic_info: String::new(),        
            fsm_state: State::Connecting,
            fsm_events: Events::NO_EVENT,
            info: serde_json::Value::Null,
            default_client: None,
            operational_client: None,
        };
        return obj;
    }


    pub fn set_info(&mut self, info: serde_json::Value) {
        self.info = info;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
        self.update_topics();
    }
    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    
    pub fn set_dev_name(&mut self, dev_name: String) {
        self.dev_name = dev_name;
        self.update_topics();
    }
    pub fn get_dev_name(&self) -> &String {
        return &self.dev_name;
    }

    pub fn set_bench_name(&mut self, bench_name: String) {
        self.bench_name = bench_name;
        self.update_topics();
    }
    pub fn get_bench_name(&self) -> &String {
        return &self.bench_name;
    }

    pub fn current_state(&self) -> &State {
        return &self.fsm_state;
    }

    pub fn events(&mut self) -> &mut Events {
        return &mut self.fsm_events;
    }

    pub fn clear_events(&mut self) {
        self.fsm_events = Events::NO_EVENT;
    }
    
    /// Move to a new state
    pub fn move_to_state(&mut self, state: State) {
        self.fsm_state = state;
        tracing::debug!("Move to state {:?}", self.fsm_state);
    }

    /// Update topics after a name change
    fn update_topics(&mut self) {
        self.topic_base = format!("pza/{}/{}/{}", self.bench_name, self.dev_name, self.name);
        self.topic_cmds = format!("{}/cmds", self.topic_base);
        self.topic_atts = format!("{}/atts", self.topic_base);
        self.topic_info = format!("{}/info", self.topic_atts);
    }


    pub fn add_client(&mut self, client: AsyncClient) {
        // self.clients.push_back(client);
    }

    /// Get the base topic
    pub async fn publish(&self, topic: &str, payload: &str, retain: bool) {
        println!("Publishing to topic: {}", topic);
        // for client in self.clients.iter() {
        //     println!("  +");
        //     client.publish(topic, rumqttc::QoS::AtLeastOnce, retain, payload).await.unwrap();
        // }
    }

    /// 
    pub async fn publish_info(&self) {
        self.publish(&self.topic_info, self.info.to_string().as_str(), false).await;
    }

}
