use std::collections::LinkedList;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::subscription::Request as SubscriptionRequest;
use crate::connection::LinkInterfaceHandle;

use crate::subscription::Message as SubscriptionMessage;

use bitflags::bitflags;


bitflags! {
    
    #[derive(Copy, Clone, Debug)]
    pub struct Events: u32 {
        const NO_EVENT                  = 0b00000000;
        const CONNECTION_UP             = 0b00000001;
        const CONNECTION_DOWN           = 0b00000010;
        const STATE_SUCCESS             = 0b00000100;
        const ERROR                     = 0b10000000;
    }
}



use async_trait::async_trait;
pub enum Event {
    NoEvent,
    ConnectionUp,
    ConnectionDown,
    InitializationOk,
    InitializationFailed,
}


#[derive(Clone, Debug)]
enum State {
    Connecting,
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

    /// Current state
    state: State,

    pub events: Events
}
pub type SafeData = Arc<Mutex<Data>>;

impl Data {
    pub fn new() -> Data {
        return Data {
            state: State::Connecting,
            events: Events::NO_EVENT
        }
    }


    fn current_state(&self) -> &State {
        return &self.state;
    }


    fn events(&self) -> &Events {
        return &self.events;
    }
    
    /// Move to a new state
    fn move_to_state(&mut self, state: State) {
        self.state = state;
        tracing::debug!("Move to state {:?}", self.state);
    }


}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

#[async_trait]
pub trait StateImplementations : Send {


    /// Poll events
    async fn poll_events(&self) -> Vec<Event>;

    async fn connecting(&self);
    async fn initializating(&self);
    async fn running(&self);
    async fn error(&self);

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
    data: SafeData,

    /// State Implementations
    impls: Box<dyn StateImplementations>,

}

impl Fsm {

    ///
    /// 
    pub fn new(data: SafeData, impls: Box<dyn StateImplementations>) -> Fsm {
        Fsm {
            data: data,
            impls: impls,
        }
    }


    ///
    ///
    pub async fn run_once(&mut self) {


        // for link in self.links.iter_mut() {
            
        //     let msg = link.rx.try_recv();
        //     match msg {
        //         Ok(msg) => {
        //             self.impls.process(&msg).await;
        //         },
        //         Err(e) => {
        //             // tracing::warn!("Error: {:?}", e);
        //         }
        //     }
        // }
        
        // Get state but do not keep the lock
        let state = self.data.lock().await.current_state().clone();
        match state {
            State::Connecting => {
                // Execute state
                self.impls.connecting().await;
                
                // Manage transitions
                let evs = self.data.lock().await.events().clone();

                // If connection up, go to running state
                if evs.contains(Events::CONNECTION_UP) && !evs.contains(Events::ERROR) {
                    self.data.lock().await.move_to_state(State::Running);
                }
                // else {
                //     tracing::debug!("{:?}", evs);
                // }
            },
            State::Running => {
                // wait for error
            },
            _ => {
                // do nothing
            }
        }

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

    async fn process(&self, data: &SafeData, msg: &SubscriptionMessage);

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
    data: SafeData,

    /// 
    impls: Box<dyn HandlerImplementations>,
    
    // links interface handles
    links: LinkedList<LinkInterfaceHandle>
}

impl Listener {
    
    fn new(data: SafeData, impls: Box<dyn HandlerImplementations>) -> Listener {
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


}

