
use lazy_static::lazy_static;


use crate::GateLogger;

lazy_static! {
    static ref GATE : tokio::sync::Mutex<Gate> 
        = tokio::sync::Mutex::new(Gate { instances: HashMap::new() });
}

// get should return an error message
pub async fn get(config: &Config) -> Result<TtyConnector, PlatformError> {
    let mut gate = GATE.lock().await;
    gate.get(config)
}


/// Main entry point to acces connectors
/// 
struct Gate {
    instances: HashMap<String, TtyConnector>
}

impl Gate {

    fn get(&mut self, config: &Config) -> Result<TtyConnector, PlatformError> {



    }

}