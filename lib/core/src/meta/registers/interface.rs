use std::sync::Arc;
use tokio::sync::Mutex;
use super::RegistersActions;
use super::settings::MetaSettings;

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
        let capacity = settings.number_of_register as usize;
        return MetaInterface {
            settings: settings,
            values: Vec::with_capacity(capacity),
            timestamps: Vec::with_capacity(capacity),
            actions: actions
        }
    }
    
    pub fn new_thread_safe(settings: MetaSettings, actions: Box<dyn RegistersActions>) -> ThreadSafeMetaInterface {
        return Arc::new(Mutex::new( MetaInterface::new(settings, actions) ));
    }
}

