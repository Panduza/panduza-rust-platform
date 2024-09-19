use std::sync::Arc;

use tokio::sync::Mutex;

use crate::MessageDispatcher;

type MessageEventLoop = rumqttc::EventLoop;

pub struct MessageEngine {
    message_dispatcher: Arc<Mutex<MessageDispatcher>>,
    message_event_loop: MessageEventLoop,
}

impl MessageEngine {
    pub fn new(
        message_dispatcher: Arc<Mutex<MessageDispatcher>>,
        message_event_loop: MessageEventLoop,
    ) -> MessageEngine {
        MessageEngine {
            message_dispatcher: message_dispatcher,
            message_event_loop: message_event_loop,
        }
    }

    pub async fn run(&mut self) {
        while let Ok(event) = self.message_event_loop.poll().await {
            // println!("Notification = {:?}", event);
            // match notification {
            //     Ok(event) => {
            match event {
                rumqttc::Event::Incoming(incoming) => {
                    // println!("Incoming = {:?}", incoming);

                    match incoming {
                        // rumqttc::Packet::Connect(_) => todo!(),
                        // rumqttc::Packet::ConnAck(_) => todo!(),
                        rumqttc::Packet::Publish(packet) => {
                            // let payload = packet.payload;
                            // let payload_str = std::str::from_utf8(&payload).unwrap();
                            // println!("Received = {:?} {:?}", payload_str, packet.topic);

                            self.message_dispatcher
                                .lock()
                                .await
                                .trigger_on_change(&packet.topic, &packet.payload)
                                .await;
                        }
                        // rumqttc::Packet::PubAck(_) => todo!(),
                        // rumqttc::Packet::PubRec(_) => todo!(),
                        // rumqttc::Packet::PubRel(_) => todo!(),
                        // rumqttc::Packet::PubComp(_) => todo!(),
                        // rumqttc::Packet::Subscribe(_) => todo!(),
                        // rumqttc::Packet::SubAck(_) => todo!(),
                        // rumqttc::Packet::Unsubscribe(_) => todo!(),
                        // rumqttc::Packet::UnsubAck(_) => todo!(),
                        // rumqttc::Packet::PingReq => todo!(),
                        // rumqttc::Packet::PingResp => todo!(),
                        // rumqttc::Packet::Disconnect => todo!(),
                        _ => {}
                    }
                }
                rumqttc::Event::Outgoing(outgoing) => {
                    // println!("Outgoing = {:?}", outgoing);
                    match outgoing {
                        // rumqttc::Outgoing::Publish(packet) => {
                        //     // println!("Publish = {:?}", packet);
                        // }
                        // rumqttc::Outgoing::Subscribe(p) => {
                        //     // println!("Subscribe = {:?}", p);
                        // }
                        // rumqttc::Outgoing::Unsubscribe(_) => todo!(),
                        // rumqttc::Outgoing::PubAck(_) => todo!(),
                        // rumqttc::Outgoing::PubRec(_) => todo!(),
                        // rumqttc::Outgoing::PubRel(_) => todo!(),
                        // rumqttc::Outgoing::PubComp(_) => todo!(),
                        // rumqttc::Outgoing::PingReq => todo!(),
                        // rumqttc::Outgoing::PingResp => todo!(),
                        // rumqttc::Outgoing::Disconnect => todo!(),
                        // rumqttc::Outgoing::AwaitAck(_) => todo!(),
                        _ => {}
                    }
                } // }
                  // }
                  // Err(_) => todo!(),
            }
        }
    }
}
