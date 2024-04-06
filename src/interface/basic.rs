use crate::subscription;

use super::core::AmCore;

/// Process a message with common behaviour for all interface
/// 
pub async fn process(core: &AmCore, msg: &subscription::Message) {
    match msg {
        subscription::Message::ConnectionStatus (status) => {
            if status.connected {
                core.lock().await.set_event_connection_up();
            }
            else {
                core.lock().await.set_event_connection_down();
            }
        },
        subscription::Message::Mqtt(msg) => {
            match msg.id() {
                subscription::ID_PZA => {
                    core.lock().await.publish_info().await;

                    tracing::trace!("Ackk !!!");
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
pub async fn interface_initializating(core: &AmCore)
{
    
}

/// Wait for a fsm event 
///
#[inline]
pub async fn wait_for_fsm_event(core: &AmCore)
{
    let fsm_events_notifier = core.lock().await.get_fsm_events_notifier();
    fsm_events_notifier.notified().await;
}
