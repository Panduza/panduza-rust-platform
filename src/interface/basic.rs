
use crate::subscription;

use super::AmInterface;

/// Process a message with common behaviour for all interface
/// 
pub async fn process(interface: &AmInterface, msg: &subscription::Message) {
    match msg {
        subscription::Message::ConnectionStatus (status) => {

            if status.connected {
                interface.lock().await.set_event_connection_up();
            }
            else {
                interface.lock().await.set_event_connection_down();
            }
        },
        subscription::Message::Mqtt(msg) => {
            match msg.id() {
                subscription::ID_PZA => {
                    interface.lock().await.publish_info().await;
                },
                // subscription::ID_PZA_CMDS_SET => {
                //     // interface.lock().await.publish_info().await;

                //     println!("BpcSubscriber::process: {:?}", msg.topic());
                //     println!("BpcSubscriber::process: {:?}", msg.payload());

                //     let payload = msg.payload();
                //     let oo = serde_json::from_slice::<Value>(payload).unwrap();
                //     let o = oo.as_object().unwrap();


                //     for (attribute_name, fields) in o.iter() {

                //         for (field_name, field_data) in fields.as_object().unwrap().iter() {


                //             if field_data.is_boolean() {
                //                 interface.lock().await.update_attribute_with_bool(
                //                     &attribute_name, &field_name, field_data.as_bool().unwrap());
                //             }
                //             else if field_data.is_f64() {
                //                 interface.lock().await.update_attribute_with_f64(
                //                     &attribute_name, &field_name, field_data.as_f64().unwrap());
                //             }
                //             else if field_data.is_string() {
                //                 interface.lock().await.update_attribute_with_string(
                //                     &attribute_name, &field_name, &String::from(field_data.as_str().unwrap()) );
                //             }

                //         }



                //     }


                // },
                _ => {
                    // not managed by the common level
                }
            }
        }
    }
}




/// Interface initializating
///
#[inline]
pub async fn interface_initializating(_interface: &AmInterface)
{
    
}

/// Wait for a fsm event 
///
#[inline]
pub async fn wait_for_fsm_event(interface: &AmInterface)
{
    let fsm_events_notifier = interface.lock().await.get_fsm_events_notifier();
    fsm_events_notifier.notified().await;
}
