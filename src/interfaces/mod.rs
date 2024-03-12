


enum States {
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


struct Interface {
    state: State,
    callbacks: StateCallbacks
}


impl Interface {

    pub fn new() -> Interface {
        Interface {}
    }

    /// Function to start the work in a task
    /// 
    pub async fn start(&mut self, task_pool: &mut tokio::task::JoinSet<()>) {
        let abort = task_pool.spawn(self.work());
    }

    ///
    pub async fn work(&mut self) {

        loop {
            
            tracing::info!("Starting interface");

            match self.state {
                States::NotConnected => {
                    // wait for connection
                },
                States::Initializing => {
                    // wait for init
                },
                States::Running => {
                    // wait for error
                },
                States::Error => {
                    // wait for connection
                }
            }
        }
    }

}

