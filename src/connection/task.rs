use super::ThreadSafeConnection;
use crate::platform::TaskResult;

/// Task that run the connection
///
async fn task(connection: ThreadSafeConnection) -> TaskResult {

    // Take the ownership of the connection while the connection task is running
    let connection_event_loop = connection.lock().await.event_loop().clone();
    let mut ev = connection_event_loop.lock().expect("Failed to lock connection event loop");

    // Event loop mangement
    loop {
        // Poll the connection event loop to get messages
        while let Ok(connection_event) = ev.poll().await {
        //     // Debug log
        //     tracing::trace!(
        //         class = "Connection",
        //         cname = conneciton_name,
        //         "{:?}",
        //         notification
        //     );

        //     // Check notification
        //     match notification {
        //         rumqttc::Event::Incoming(incoming) => {
        //             Connection::process_incoming_packet(lm.clone(), &incoming).await;
        //         }
        //         // rumqttc::Event::Outgoing(outgoing) => {
        //         //     match outgoing {
        //         //         rumqttc::Outgoing::Subscribe(subscribe) => {
        //         //             println!("Subscribe = {:?}", subscribe);
        //         //         },
        //         //         _ => {
        //         //             // println!("Outgoing = {:?}", outgoing);
        //         //         }
        //         //         // rumqttc::Outgoing::Publish(_) => todo!(),
        //         //         // rumqttc::Outgoing::Unsubscribe(_) => todo!(),
        //         //         // rumqttc::Outgoing::PubAck(_) => todo!(),
        //         //         // rumqttc::Outgoing::PubRec(_) => todo!(),
        //         //         // rumqttc::Outgoing::PubRel(_) => todo!(),
        //         //         // rumqttc::Outgoing::PubComp(_) => todo!(),
        //         //         // rumqttc::Outgoing::PingReq => todo!(),
        //         //         // rumqttc::Outgoing::PingResp => todo!(),
        //         //         // rumqttc::Outgoing::Disconnect => todo!(),
        //         //         // rumqttc::Outgoing::AwaitAck(_) => todo!(),
        //         //     }
        //         //     // println!("Outgoing = {:?}", outgoing);
        //         // }
        //         _ => {
        //             // println!("Received = {:?}", notification);
        //         }
        //     }

        //     // \todo: check for link manager events and initialize the new created links
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
