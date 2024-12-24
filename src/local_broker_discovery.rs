use panduza_platform_core::{log_info, Error};
use serde_json::json;
use tokio::net::UdpSocket;

/// Start the task for Panduza Local Broker Discovery (PLBD)
///
pub async fn task(platform_name: Option<String>) -> Result<(), Error> {
    //
    //
    let logger = panduza_platform_core::Logger::new_for_platform();

    // If panic send the message expected
    // start the connection
    let socket = UdpSocket::bind("0.0.0.0:53035")
        .await
        .expect("creation local discovery socket failed");
    log_info!(logger, "Local discovery service start");

    // Go look the platform_name in the connection.json

    let json_reply: String = format!(
        "{{
        \"platform\": {{
            \"name\": \"{}\",
        }}
    }}",
        platform_name.unwrap_or("platform".to_string())
    );

    let mut buf = [0; 1024];
    let json_reply_bytes = json_reply.as_bytes();

    loop {
        // Receive request and answer it
        // Error who didn't depend of the user so user unwrap or expects
        // if message
        let result_recv = socket.recv_from(&mut buf).await;
        match result_recv {
            Ok(msg_content) => {
                let (nbr_bytes, src_addr) = msg_content;

                let filled_buf = &mut buf[..nbr_bytes];

                // need to manage if conversion from utf8 fail (with log)
                let buf_utf8 = std::str::from_utf8(&filled_buf);

                match buf_utf8 {
                    Ok(buf) => {
                        let json_content: Result<serde_json::Value, serde_json::Error> =
                            serde_json::from_str(&buf);
                        match json_content {
                            Ok(content) => {
                                if content["search"] != json!(true) {
                                    tracing::trace!(
                                        class = "Platform",
                                        "Local discovery request message incorrect"
                                    );
                                    continue;
                                }
                                let _ = socket.send_to(json_reply_bytes, &src_addr).await;
                                tracing::trace!(
                                    class = "Platform",
                                    "Local discovery reply send success"
                                );
                            }
                            Err(_e) => {
                                tracing::trace!(
                                    class = "Platform",
                                    "Json request not correctly formatted"
                                );
                            }
                        }
                    }
                    Err(_e) => {
                        tracing::trace!(
                            class = "Platform",
                            "Request need to be send to UTF-8 format"
                        );
                    }
                }
            }
            Err(e) => {
                tracing::warn!("Local discovery error: {:?}", e);
            }
        }
    }
}
