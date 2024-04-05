use serde_json;
use serde_json::json;

use std::sync::Arc;

use rumqttc::AsyncClient;

use tokio::sync::Mutex;
use tokio::sync::Notify;

use crate::interface::fsm::State;
use crate::interface::fsm::Events;




struct JsonAttribute {
    // 
    name: String,
    
    // 
    data: serde_json::Value,
}

impl JsonAttribute {
    pub fn new<A: Into<String>>(name: A) -> JsonAttribute {

        let name_str = name.into();

        let data = json!({
            name_str.clone(): {}
        });

        return JsonAttribute {
            name: name_str,
            data: data,
        };
    }

    pub fn update_field(&mut self, field: &str, value: serde_json::Value) {
        let n = self.name.clone();
        let d = self.data.get_mut(n);
        if d.is_none() {
            return;
        }
        d.unwrap().as_object_mut().unwrap().insert(field.to_string(), value);
        
    }

    pub fn to_str(&self) -> String {
        return self.data.to_string();
    }

}





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
    
    // -- CLIENTS --
    client: AsyncClient,

    // -- FSM --
    /// Current state
    fsm_state: State,
    /// Events
    fsm_events: Events,
    /// Notifier for events
    fsm_events_notifier: Arc<Notify>,

    // -- ATTRIBUTE --
    /// Interface Indentity Info
    info: JsonAttribute,

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

        let mut info = JsonAttribute::new("info");
        info.update_field("type", serde_json::Value::String(itype.into()));
        info.update_field("version", serde_json::Value::String(version.into()));

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
            info: info,
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



    /// Get the base topic
    /// 
    pub async fn publish(&self, topic: &str, payload: &str, retain: bool) {
        println!("Publishing to topic: {}", topic);
        self.client.publish(topic, rumqttc::QoS::AtLeastOnce, retain, payload).await.unwrap();
    }

    /// Publish the info
    /// 
    pub async fn publish_info(&self) {
        println!("Publishing info: {:?}", self.info.to_str());
        self.publish(&self.topic_info, &self.info.to_str(), false).await;
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
