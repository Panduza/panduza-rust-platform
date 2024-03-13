use async_trait::async_trait;

enum Event {
    NoEvent,
    ConnectionUp,
    ConnectionDown,
    InitializationOk,
    InitializationFailed,
}

enum State {
    Connecting,
    Mounting,
    Running,
    Unmounting,
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
trait StateImplementations {

    async fn connecting(&self) -> Result<Event, String>;
        // state == function that return an event
}




/// Interface finite state machine
/// 
pub struct Fsm {
    state: State,
    states_implementations: Box<dyn StateImplementations>
}


impl Fsm {

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
                match self.states_implementations.connecting().await {
                    Ok(event) => {
                        match event {
                            Event::ConnectionUp => {
                                // self.state = State::Initializing;
                            },
                            _ => {
                                // do nothing
                            }
                        }
                    },
                    Err(e) => {
                        // do nothing
                    }
                }
            },
            // State::Initializing => {
            //     // wait for init
            // },
            // State::Running => {
            //     // wait for error
            // },
            // State::Error => {
            //     // wait for connection
            // }
            _ => {
                // do nothing
            }
        }

    }

}




