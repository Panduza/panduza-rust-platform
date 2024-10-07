use panduza_platform_core::{Error, PlatformLogger};
use std::sync::Arc;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::sync::Notify;

///
///
///
static REQUEST_CHANNEL_SIZE: usize = 64;

pub enum ServiceRequest {
    Boot,
    LoadPlugins,
    StartBroker,
}

// run actions when request arrives
//
pub struct Services {
    logger: PlatformLogger,

    request_sender: Sender<ServiceRequest>,
    request_receiver: Option<Receiver<ServiceRequest>>,
}

impl Services {
    ///
    ///
    pub fn new() -> Self {
        let (rqst_tx, rqst_rx) = channel::<ServiceRequest>(REQUEST_CHANNEL_SIZE);
        Self {
            logger: PlatformLogger::new(),
            request_sender: rqst_tx.clone(),
            request_receiver: Some(rqst_rx),
        }
    }

    ///
    ///
    pub async fn run_task(mut self) -> Result<(), Error> {
        let mut request_receiver = self.request_receiver.take().unwrap();
        loop {
            let request = request_receiver
                .recv()
                .await
                .ok_or(Error::ChannelError(format!("The channel seems broken")))?;
            match request {
                ServiceRequest::Boot => todo!(),
                ServiceRequest::LoadPlugins => todo!(),
                ServiceRequest::StartBroker => todo!(),
            }
        }
    }
}
