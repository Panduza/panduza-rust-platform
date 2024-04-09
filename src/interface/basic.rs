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
pub async fn interface_initializating(interface: &AmInterface)
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
