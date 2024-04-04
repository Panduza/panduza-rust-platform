use serde_json;
use serde_json::json;

use std::sync::Arc;

use rumqttc::AsyncClient;

use tokio::sync::Mutex;
use tokio::sync::Notify;

use crate::interface::fsm::State;
use crate::interface::fsm::Events;




struct JsonAttribute {
    /// Shared interface data
    core: AmCore,

    // 
    name: String,
    
    // 
    data: serde_json::Value,
}

impl JsonAttribute {
    pub fn new<A: Into<String>>(core: AmCore, name: A) -> JsonAttribute {

        let name_str = name.into();

        let data = json!({
            name_str.clone(): {}
        });

        return JsonAttribute {
            core: core,
            name: name_str,
            data: data,
        };
    }

    pub fn update_field(&mut self, field: &str, value: serde_json::Value) {
        self.data[field] = value;
    }

    pub async fn publish(&self, retain: bool) {
        let payload = self.data.to_string();
        self.core.lock().await.publish(&self.core.lock().await.topic_info, payload.as_str(), retain).await;
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
    info: Option<JsonAttribute>,

}
pub type AmCore = Arc<Mutex<Core>>;

impl Core {

    /// Create a new instance of the Core
    ///
    fn new<A: Into<String>, B: Into<String>, C: Into<String>>
        (name: A, dev_name: B, bench_name: C, client: AsyncClient)
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
            info: None,
        };
        obj.update_topics();
        return obj;
    }

    /// Create a new instance of the Core
    /// 
    pub fn new_am<A: Into<String>, B: Into<String>, C: Into<String>>
        (name: A, dev_name: B, bench_name: C, client: AsyncClient)
            -> AmCore
    {
        return Arc::new(Mutex::new(
            Core::new(name, dev_name, bench_name, client)
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
    pub async fn publish(&self, topic: &str, payload: &str, retain: bool) {
        println!("Publishing to topic: {}", topic);


        
        self.client.publish(topic, rumqttc::QoS::AtLeastOnce, retain, payload).await.unwrap();
        
        
    }

    /// 
    /// 
    pub async fn publish_info(&self) {
        // self.publish(&self.topic_info, self.info.to_string().as_str(), false).await;
    }

    /// Init the info attribute
    /// 
    pub fn init_info(&mut self, core: AmCore) {
        self.info = Some(JsonAttribute::new(core, "info"));
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
