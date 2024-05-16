use std::sync::atomic::{AtomicBool, Ordering};
use super::logger::Logger;
use super::ThreadSafeConnection;
use crate::platform::TaskResult;
use crate::link::ThreadSafeLinkManager;
use crate::subscription;

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Task that run the connection
///
pub async fn task(connection: ThreadSafeConnection) -> TaskResult {

    // Atomic boolean to check if the connection is connected
    let mut is_connected = AtomicBool::new(false);

    // --- Take from the connection required elements to run the task ---
    // Take logger
    let logger = connection.lock().await.logger().clone();
    // Take the ownership of the connection while the connection task is running
    let connection_event_loop = connection.lock().await.event_loop().clone();
    let mut c_event_loop = connection_event_loop.lock().await;
    // Take the link manager
    let link_manager = connection.lock().await.link_manager().clone();

    // Event loop mangement
    loop {
        // Poll the connection event loop to get messages
        while let Ok(connection_event) = c_event_loop.poll().await {
            // Log the event
            logger.log_trace(format!("Event received {:?}", connection_event));

            // Check event
            match connection_event {
                // Incoming event
                rumqttc::Event::Incoming(incoming) => {
                    match incoming {
                        // Connection ack
                        rumqttc::Incoming::ConnAck(ack) => {
                            process_incoming_conn_ack(&logger, &link_manager, &ack, &mut is_connected).await;
                        },
                        rumqttc::Incoming::Publish(publish) => {
                            // For each link with interfaces, check if the topic matches a filter
                            // then send the message to the interface
                            for link in link_manager.lock().await.links_as_mut().iter_mut() {
                                for filter in link.filters().iter() {
                                    if filter.match_topic(&publish.topic) {
                                        let message = 
                                            subscription::Message::from_filter_and_publish_packet(filter, &publish);
                
                                        // tracing::trace!(
                                        //     "Sending message to interface {}", message);
                
                
                                        let r = link.tx().send(message).await;
                                        if r.is_err() {
                                            println!("Error sending message to interface {}",
                                                r.err().unwrap());
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                        }
                    }
                },
                rumqttc::Event::Outgoing(outgoing) => {
                    // process_outgoing_packet(&logger, &link_manager, &outgoing).await;
                },
                _ => {
                    logger.log_warn(format!("UNEXPECTED Event received !!! {:?}", connection_event));
                }
            }

            // 
            link_manager.lock().await.process_new_links(&is_connected).await;
        }

        // // Here the broker is disconnected
        // tracing::warn!(
        //     class = "Connection",
        //     cname = conneciton_name,
        //     "Broker disconnected, trying to reconnect"
        // );

        // let message = subscription::Message::new_connection_status(false);
        // // let r = link.tx.send(message).await;
    }
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Process incoming connection ack
/// 
async fn process_incoming_conn_ack(
    logger: &Logger,
    link_manager: &ThreadSafeLinkManager,
    conn_ack: &rumqttc::ConnAck,
    is_connected: &mut AtomicBool
)
{
    match conn_ack.code {
        rumqttc::ConnectReturnCode::Success => {
            is_connected.store(true, Ordering::Relaxed);
            link_manager.lock().await.send_to_all(subscription::Message::new_connection_status(true)).await;
        },
        _ => {
            logger.log_warn(format!("Connection failed {:?}", conn_ack.code));
        }
    }
}

