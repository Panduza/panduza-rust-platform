use serde_json;

use std::sync::Arc;

use rumqttc::AsyncClient;

use tokio::sync::Mutex;
use tokio::sync::Notify;

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
    /// Notifier for events
    fsm_events_notifier: Arc<Notify>,


    // -- CLIENTS --
    
    // use both
    // operational only

    default_client: Option<AsyncClient>,
    operational_client: Option<AsyncClient>
}
pub type AmCore = Arc<Mutex<Core>>;

impl Core {

    /// Create a new instance of the Core
    ///
    pub fn new<A: Into<String>>(name: A) -> Core {
        let mut obj = Core {
            name: name.into(),
            dev_name: String::new(),
            bench_name: String::new(),
            topic_base: String::new(),
            topic_cmds: String::new(),
            topic_atts: String::new(),
            topic_info: String::new(),
            fsm_state: State::Connecting,
            fsm_events: Events::NO_EVENT,
            fsm_events_notifier: Arc::new(Notify::new()),
            info: serde_json::Value::Null,
            default_client: None,
            operational_client: None
        };
        return obj;
    }

    /// Get the name of the interface
    /// 
    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    /// Set the names of the interface device and bench
    /// 
    pub fn set_dev_and_bench_names<A: Into<String>, B: Into<String>>(&mut self, dev_name: A, bench_name: B) {
        self.dev_name = dev_name.into();
        self.bench_name = bench_name.into();
        self.update_topics();
    }

    /// Get the name of the device
    /// 
    pub fn get_dev_name(&self) -> &String {
        return &self.dev_name;
    }

    /// Get the name of the bench
    /// 
    pub fn get_bench_name(&self) -> &String {
        return &self.bench_name;
    }






    pub fn set_info(&mut self, info: serde_json::Value) {
        self.info = info;
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
    /// 
    pub fn move_to_state(&mut self, state: State) {
        let previous = self.fsm_state.clone();
        self.fsm_state = state;
        tracing::info!(
            class="Interface", 
            bname= self.bench_name, dname= self.dev_name, iname= self.name,
                "State changed {:?} => {:?}", previous, self.fsm_state);
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

    /// Update topics after a name change
    fn update_topics(&mut self) {
        self.topic_base = format!("pza/{}/{}/{}", self.bench_name, self.dev_name, self.name);
        self.topic_cmds = format!("{}/cmds", self.topic_base);
        self.topic_atts = format!("{}/atts", self.topic_base);
        self.topic_info = format!("{}/info", self.topic_atts);
    }

    ///
    pub fn set_default_client(&mut self, client: AsyncClient) {
        self.default_client = Some(client);
    }

    ///
    pub fn set_operational_client(&mut self, client: AsyncClient) {
        self.operational_client = Some(client);
    }

    /// Get the base topic
    pub async fn publish(&self, topic: &str, payload: &str, retain: bool) {
        println!("Publishing to topic: {}", topic);


        // if let Some(client) = &self.default_client {
        //     client.publish(topic, rumqttc::QoS::AtLeastOnce, retain, payload).await.unwrap();
        // }
        
    }

    /// 
    /// 
    pub async fn publish_info(&self) {
        self.publish(&self.topic_info, self.info.to_string().as_str(), false).await;
    }


}
