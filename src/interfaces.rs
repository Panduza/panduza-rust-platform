use async_trait::async_trait;

enum Event {
    NoEvent,
    ConnectionUp,
    ConnectionDown,
    InitializationOk,
    InitializationFailed,
}

enum State {
    NotConnected,
    Initializing,
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
trait StateImplementations {

    async fn connecting(&self) -> Event;
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
            state: State::NotConnected,
            states_implementations: states_implementations
        }
    }

}

//     ///
//     pub async fn poll(&mut self) {

//         loop {
            
//             tracing::info!("Starting interface");

//             match self.state {
//                 State::NotConnected => {
//                     // wait for connection
//                     tracing::info!("Waiting for connection");

//                 },
//                 State::Initializing => {
//                     // wait for init
//                 },
//                 State::Running => {
//                     // wait for error
//                 },
//                 State::Error => {
//                     // wait for connection
//                 }
//             }
//         }
//     }

// }

