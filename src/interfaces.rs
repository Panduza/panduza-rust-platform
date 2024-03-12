


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
trait StateCallbacks {

}


pub struct Interface {
    state: State,
    // callbacks: StateCallbacks
}


impl Interface {

    pub fn new() -> Interface {
        Interface {
            state: State::NotConnected
        }
    }



    ///
    pub async fn poll(&mut self) {

        loop {
            
            tracing::info!("Starting interface");

            match self.state {
                State::NotConnected => {
                    // wait for connection
                    tracing::info!("Waiting for connection");

                },
                State::Initializing => {
                    // wait for init
                },
                State::Running => {
                    // wait for error
                },
                State::Error => {
                    // wait for connection
                }
            }
        }
    }

}

