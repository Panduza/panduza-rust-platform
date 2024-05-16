use super::ThreadSafeConnection;
use crate::platform::TaskResult;
use crate::link::ThreadSafeLinkManager;

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Task that run the connection
///
pub async fn task(connection: ThreadSafeConnection) -> TaskResult {

    // --- Take from the connection required elements to run the task ---
    // Take logger
    let logger = connection.lock().await.logger().clone();
    // Take the ownership of the connection while the connection task is running
    let connection_event_loop = connection.lock().await.event_loop().clone();
    let mut ev = connection_event_loop.lock().await;
    // Take the link manager
    let link_manager = connection.lock().await.link_manager().clone();

    // Event loop mangement
    loop {
        // Poll the connection event loop to get messages
        while let Ok(connection_event) = ev.poll().await {
            // Log the event
            logger.log_trace(format!("Event received {:?}", connection_event));

            // Check event
            match connection_event {
                rumqttc::Event::Incoming(incoming) => {
                    process_incoming_packet(&link_manager, &incoming).await;
                },
                rumqttc::Event::Outgoing(outgoing) => {
                    process_outgoing_packet(&link_manager, &outgoing).await;
                },
                _ => {
                    logger.log_warn(format!("UNEXPECTED Event received !!! {:?}", connection_event));
                }
            }

            // \todo: check for link manager events and initialize the new created links
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

/// Process incoming packets
/// 
async fn process_incoming_packet(link_manager: &ThreadSafeLinkManager, packet: &rumqttc::Packet) {

    match packet {
        rumqttc::Incoming::ConnAck(ack) => {
            println!("ConnAck = {:?}", ack);
            // lm.lock().await.send_to_all(subscription::Message::new_connection_status(true)).await;
        },
    //     // rumqttc::Packet::SubAck(ack) => {
    //     //     println!("SubAck = {:?}", ack);
    //     // },
    //     rumqttc::Incoming::Publish(publish) => {
    //         // For each link with interfaces, check if the topic matches a filter
    //         // then send the message to the interface
    //         for link in lm.lock().await.links_as_mut().iter_mut() {
    //             for filter in link.filters().iter() {
    //                 if filter.match_topic(&publish.topic) {
    //                     let message = 
    //                         subscription::Message::from_filter_and_publish_packet(filter, publish);

    //                     // tracing::trace!(
    //                     //     "Sending message to interface {}", message);


    //                     let r = link.tx().send(message).await;
    //                     if r.is_err() {
    //                         println!("Error sending message to interface {}",
    //                             r.err().unwrap());
    //                     }
    //                 }
    //             }
    //         }
    //     }
        _ => {
            // println!("? = {:?}", packet);
        }
    }
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Process outgoing packets
/// 
async fn process_outgoing_packet(link_manager: &ThreadSafeLinkManager, outgoing: &rumqttc::Outgoing) {


}

