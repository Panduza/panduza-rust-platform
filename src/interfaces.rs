use std::collections::LinkedList;

use crate::subscription::Request as SubscriptionRequest;
use crate::connection::LinkInterfaceHandle;

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


#[async_trait]
pub trait StateImplementations : Send {



    async fn get_subscription_requests(&self) -> Vec<SubscriptionRequest>;

    /// Poll events
    async fn poll_events(&self) -> Vec<Event>;

    async fn enter_connecting(&self, links: &LinkedList<LinkInterfaceHandle>);
    async fn state_connecting(&self);
    async fn leave_connecting(&self);

    async fn enter_running(&self);
    async fn state_running(&self);
    async fn leave_running(&self);

        // state == function that return an event
}




/// Interface finite state machine
/// 
pub struct Fsm {
    state: State,
    states_implementations: Box<dyn StateImplementations>,

    // links interface handles
    links: LinkedList<LinkInterfaceHandle>
}

// struct SubInter {
//     fsm: Fsm
// }


impl Fsm {

    ///
    /// 
    pub fn new(states_implementations: Box<dyn StateImplementations>) -> Fsm {
        Fsm {
            state: State::Connecting,
            states_implementations: states_implementations,
            links: LinkedList::new()
        }
    }

    pub fn add_link(&mut self, link: LinkInterfaceHandle) {
        self.links.push_back(link);
    }

    ///
    /// 
    pub async fn run_once(&mut self) {

        match self.state {
            State::Connecting => {
                self.states_implementations.enter_connecting(&self.links).await;
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




