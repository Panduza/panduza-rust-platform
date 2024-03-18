use std::collections::LinkedList;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::subscription::Request as SubscriptionRequest;
use crate::connection::LinkInterfaceHandle;

use crate::subscription::Message as SubscriptionMessage;


use async_trait::async_trait;
pub enum Event {
    NoEvent,
    ConnectionUp,
    ConnectionDown,
    InitializationOk,
    InitializationFailed,
}

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

#[async_trait]
pub trait StateImplementations : Send {


    /// Poll events
    async fn poll_events(&self) -> Vec<Event>;

    async fn enter_connecting(&self);
    async fn state_connecting(&self);
    async fn leave_connecting(&self);

    async fn enter_running(&self);
    async fn state_running(&self);
    async fn leave_running(&self);

        // state == function that return an event
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Interface finite state machine
///
pub struct Fsm {
    state: State,
    states_implementations: Box<dyn StateImplementations>,

}

impl Fsm {

    ///
    /// 
    pub fn new(states_implementations: Box<dyn StateImplementations>) -> Fsm {
        Fsm {
            state: State::Connecting,
            states_implementations: states_implementations,
        }
    }


    ///
    ///
    pub async fn run_once(&mut self) {


        // for link in self.links.iter_mut() {
            
        //     let msg = link.rx.try_recv();
        //     match msg {
        //         Ok(msg) => {
        //             self.states_implementations.process(&msg).await;
        //         },
        //         Err(e) => {
        //             // tracing::warn!("Error: {:?}", e);
        //         }
        //     }
        // }

        match self.state {
            State::Connecting => {
                self.states_implementations.enter_connecting().await;
                // match self.states_implementations.state_connecting().await {
                //     Ok(event) => {
                //         match event {
                //             Event::ConnectionUp => {
                //                 self.state = State::Mounting;
                //             },
                //             _ => {
                //                 // do nothing
                //             }
                //         }
                //     },
                //     Err(e) => {
                //         // do nothing
                //     }
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

    async fn process(&self, msg: &SubscriptionMessage);

}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Message handler
/// 
struct Listener {
    impls: Box<dyn HandlerImplementations>,
    
    // links interface handles
    links: LinkedList<LinkInterfaceHandle>
}

impl Listener {
    
    fn new(impls: Box<dyn HandlerImplementations>) -> Listener {
        return Listener {
            impls,
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
                    self.impls.process(&msg).await;
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
    
    ///
    /// 
    pub fn new(state_impls: Box<dyn StateImplementations>, listener_impls: Box<dyn HandlerImplementations>) -> Interface {
        return Interface {
            fsm: Arc::new(Mutex::new(Fsm::new(state_impls))),
            listener: Arc::new(Mutex::new(Listener::new(listener_impls)))
        }
    }

    ///
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

