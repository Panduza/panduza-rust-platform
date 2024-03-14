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




/// Interface finite state machine
/// 
pub struct Fsm {
    state: State,
    states_implementations: Box<dyn StateImplementations>
}


impl Fsm {

    ///
    /// 
    pub fn new(states_implementations: Box<dyn StateImplementations>) -> Fsm {
        Fsm {
            state: State::Connecting,
            states_implementations: states_implementations
        }
    }

    ///
    /// 
    pub async fn run_once(&mut self) {

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




