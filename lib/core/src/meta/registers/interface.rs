use std::sync::Arc;
use tokio::sync::Mutex;
use super::RegistersActions;
use super::settings::MetaSettings;

use serde_json::json;

/// Meta Interface
/// 
pub struct MetaInterface {
    pub settings: MetaSettings,
    pub values: Vec<u64>,
    pub timestamps: Vec<u64>,
    pub actions: Box<dyn RegistersActions>
}
type ThreadSafeMetaInterface = Arc<Mutex<MetaInterface>>;


impl MetaInterface {

    fn new(settings: MetaSettings, actions: Box<dyn RegistersActions>) -> MetaInterface {
        let map_size = settings.number_of_register as usize;
        return MetaInterface {
            settings: settings,
            values: vec![0; map_size],
            timestamps: vec![0; map_size],
            actions: actions
        }
    }
    
    pub fn new_thread_safe(settings: MetaSettings, actions: Box<dyn RegistersActions>) -> ThreadSafeMetaInterface {
        return Arc::new(Mutex::new( MetaInterface::new(settings, actions) ));
    }



    pub fn to_payload(&self) -> Vec<u8> {
        let payload = json!({
            "values": self.values,
            "timestamps": self.timestamps
        }).to_string().into_bytes();

        return payload;
    }
}

