use std::collections::LinkedList;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::subscription::Request as SubscriptionRequest;
use crate::connection::LinkInterfaceHandle;

use crate::subscription::Message as SubscriptionMessage;

use bitflags::bitflags;

use async_trait::async_trait;

bitflags! {
    
    #[derive(Copy, Clone, Debug)]
    pub struct Events: u32 {
        const NO_EVENT                  = 0b00000000;
        const CONNECTION_UP             = 0b00000001;
        const CONNECTION_DOWN           = 0b00000010;
        const INIT_DONE                 = 0b00000100;
        const ERROR                     = 0b10000000;
    }
}


impl Events {
    pub fn set_connection_up(&mut self) {
        self.insert(Events::CONNECTION_UP);
    }
    pub fn set_connection_down(&mut self) {
        self.insert(Events::CONNECTION_DOWN);
    }
    pub fn set_init_done(&mut self) {
        self.insert(Events::INIT_DONE);
    }
    pub fn set_state_error(&mut self) {
        self.insert(Events::ERROR);
    }
}




#[derive(Clone, Debug)]
enum State {
    Connecting,
    Initializating,
    Running,
    Error
}


// waiting for connection
    // connection up
// initialization
    // init ok
// run
    // fail
    // conn down
// error


// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Shared data for an interface
/// 
pub struct Data {

    name: String,
    dev_name: String,
    bench_name: String,

    topic_base: String,
    topic_cmds: String,
    topic_atts: String,

    /// Current state
    state: State,

    pub events: Events,

}
pub type SharedData = Arc<Mutex<Data>>;

impl Data {

    pub fn new() -> Data {
        return Data {
            name: String::new(),
            dev_name: String::new(),
            bench_name: String::new(),
            topic_base: String::new(),
            topic_cmds: String::new(),
            topic_atts: String::new(),        
            state: State::Connecting,
            events: Events::NO_EVENT
        }
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

    fn current_state(&self) -> &State {
        return &self.state;
    }

    fn events(&self) -> &Events {
        return &self.events;
    }

    fn clear_events(&mut self) {
        self.events = Events::NO_EVENT;
    }
    
    /// Move to a new state
    fn move_to_state(&mut self, state: State) {
        self.state = state;
        tracing::debug!("Move to state {:?}", self.state);
    }

    fn update_topics(&mut self) {
        self.topic_base = format!("");
    }




}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

#[async_trait]
pub trait StateImplementations : Send {

    async fn connecting(&self, data: &SharedData);
    async fn initializating(&self, data: &SharedData);
    async fn running(&self, data: &SharedData);
    async fn error(&self, data: &SharedData);

}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Interface finite state machine
///
pub struct Fsm {

    /// Shared state data
    data: SharedData,

    /// State Implementations
    impls: Box<dyn StateImplementations>,

}

impl Fsm {

    ///
    /// 
    pub fn new(data: SharedData, impls: Box<dyn StateImplementations>) -> Fsm {
        Fsm {
            data: data,
            impls: impls,
        }
    }

    ///
    ///
    pub async fn run_once(&mut self) {
        
        // Get state but do not keep the lock
        let state = self.data.lock().await.current_state().clone();

        // Perform state task
        match state {
            State::Connecting => {
                // Execute state
                self.impls.connecting(&self.data).await;
                
                // Manage transitions
                let evs = self.data.lock().await.events().clone();

                // If connection up, go to running state
                if evs.contains(Events::CONNECTION_UP) && !evs.contains(Events::ERROR) {
                    self.data.lock().await.move_to_state(State::Initializating);
                }
            },
            State::Initializating => {
                // Execute state
                self.impls.initializating(&self.data).await;

                // Manage transitions
                let evs = self.data.lock().await.events().clone();

                // If initialization ok, go to running state
                if evs.contains(Events::INIT_DONE) && !evs.contains(Events::ERROR) {
                    self.data.lock().await.move_to_state(State::Running);
                }
                // If error, go to error state
                else if evs.contains(Events::ERROR) {
                    self.data.lock().await.move_to_state(State::Error);
                }
            },
            State::Running => {
                // Execute state
                self.impls.running(&self.data).await;

                // Manage transitions
                let evs = self.data.lock().await.events().clone();

                // If error, go to error state
                if evs.contains(Events::ERROR) {
                    self.data.lock().await.move_to_state(State::Error);
                }
                // // If connection down, go to connecting state
                // else if evs.contains(Events::CONNECTION_DOWN) {
                //     self.data.lock().await.move_to_state(State::Connecting);
                // }
            },
            State::Error => {
                // Execute state
                self.impls.error(&self.data).await;
            }
        }

        // Clear events for next run
        self.data.lock().await.clear_events();

    }

}



// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

#[async_trait]
pub trait HandlerImplementations : Send {

    async fn get_subscription_requests(&self) -> Vec<SubscriptionRequest>;

    async fn process(&self, data: &SharedData, msg: &SubscriptionMessage);

}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Message handler
/// 
struct Listener {
    
    /// Shared state data
    data: SharedData,

    /// 
    impls: Box<dyn HandlerImplementations>,
    
    // links interface handles
    links: LinkedList<LinkInterfaceHandle>
}

impl Listener {
    
    fn new(data: SharedData, impls: Box<dyn HandlerImplementations>) -> Listener {
        return Listener {
            data: data,
            impls: impls,
            links: LinkedList::new()
        }
    }

    ///
    ///
    pub async fn get_subscription_requests(&self) -> Vec<SubscriptionRequest> {
        return self.impls.get_subscription_requests().await;
    }

    ///
    /// 
    pub fn add_link(&mut self, link: LinkInterfaceHandle) {
        self.links.push_back(link);
    }
    
    ///
    ///
    pub async fn run_once(&mut self) {
        for link in self.links.iter_mut() {
            let msg = link.rx.recv().await;
            match msg {
                Some(msg) => {
                    self.impls.process(&self.data, &msg).await;
                },
                None => {
                    // do nothing
                }
            }
        }
    }

}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------


pub struct Interface {
        
    /// Shared state data
    data: SharedData,

    fsm: Arc<Mutex<Fsm>>,
    listener: Arc<Mutex<Listener>>,
}
pub type SafeInterface = Arc<Mutex<Interface>>;


impl Interface {

    /// Create a new instance of the Interface
    /// 
    pub fn new(state_impls: Box<dyn StateImplementations>, listener_impls: Box<dyn HandlerImplementations>) -> Interface {
        let data = Arc::new(Mutex::new(Data::new()));
        return Interface {
            data: data.clone(),
            fsm: Arc::new(Mutex::new(Fsm::new(data.clone(), state_impls))),
            listener: Arc::new(Mutex::new(Listener::new(data.clone(), listener_impls)))
        }
    }

    /// Start the interface, run it into tasks
    /// 
    pub async fn start(&mut self, task_pool: &mut tokio::task::JoinSet<()>) {
        
        let fsm = self.fsm.clone();
        let listener = self.listener.clone();

        task_pool.spawn(async move {
            loop {
                fsm.lock().await.run_once().await;
            }
        });

        task_pool.spawn(async move {
            loop {
                listener.lock().await.run_once().await;
            }
        });

    }

    ///
    /// 
    pub async fn get_subscription_requests(&self) -> Vec<SubscriptionRequest> {
        return self.listener.lock().await.get_subscription_requests().await;
    }

    ///
    /// 
    pub async fn add_link(&mut self, link: LinkInterfaceHandle) {
        let mut listener = self.listener.lock().await;
        listener.add_link(link);
    }


    pub async fn set_name(&mut self, name: String) {
        self.data.lock().await.set_name(name);
    }

    pub async fn set_dev_name(&mut self, dev_name: String) {
        self.data.lock().await.set_dev_name(dev_name);
    }

    pub async fn set_bench_name(&mut self, bench_name: String) {
        self.data.lock().await.set_bench_name(bench_name);
    }



}

