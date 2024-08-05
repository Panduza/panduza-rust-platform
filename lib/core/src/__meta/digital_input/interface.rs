use std::sync::Arc;
use bytes::Bytes;
use tokio::sync::Mutex;

use std::collections::LinkedList;

use crate::attribute;
use crate::attribute::ThreadSafeAttribute;
use crate::attribute::pack_attribute_as_thread_safe;

use super::MetaActions;
// use super::settings::MetaSettings;

use serde_json::json;


pub struct CyclicOperation {
    pub interval: u64,
    pub payload: Bytes
}

/// Meta Interface
/// 
pub struct MetaInterface {
    // pub settings: MetaSettings,
    // pub values: Vec<u64>,
    // pub timestamps: Vec<u64>,
    pub actions: Box<dyn MetaActions>,

    pub cyclic_operations: Arc<Mutex<LinkedList<CyclicOperation>>>,

    pub attribute_map: ThreadSafeAttribute,
    pub attribute_settings: ThreadSafeAttribute
}
type ThreadSafeMetaInterface = Arc<Mutex<MetaInterface>>;


impl MetaInterface {

    fn new(actions: Box<dyn MetaActions>) -> MetaInterface {
        // let map_size = settings.number_of_register as usize;
        return MetaInterface {
            // values: vec![0; map_size],
            // timestamps: vec![0; map_size],
            actions: actions,
            cyclic_operations: Arc::new( Mutex::new(LinkedList::new()) ),
            attribute_map: pack_attribute_as_thread_safe(
                attribute::Attribute::A3(attribute::A3::new("map"))
            ),
            attribute_settings: pack_attribute_as_thread_safe(
                attribute::Attribute::A1(attribute::A1::new("settings"))
            ),
        }
    }
    
    pub fn new_thread_safe(actions: Box<dyn MetaActions>) -> ThreadSafeMetaInterface {
        return Arc::new(Mutex::new( MetaInterface::new(actions) ));
    }



    // pub fn to_payload(&self) -> Vec<u8> {
    //     let payload = json!({
    //         "values": self.values,
    //         "timestamps": self.timestamps
    //     }).to_string().into_bytes();

    //     return payload;
    // }
}

