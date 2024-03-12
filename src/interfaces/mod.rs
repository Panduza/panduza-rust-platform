


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
    state: State 
    // states definition
}


impl Interface {
    pub fn new() -> Interface {
        Interface {}
    }
}

