use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::attribute::RawAttribute;
use crate::interface::AmInterface;
use crate::{interface, subscription};
use crate::interface::builder::Builder as InterfaceBuilder;

use crate::Error as PlatformError;

use crate::FunctionResult as PlatformFunctionResult;

use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType, CameraFormat, FrameFormat};
use nokhwa::Camera;
use nokhwa_core::pixel_format::RgbFormat;

#[async_trait]
pub trait VideoActions: Send + Sync {

    /// Initialize the interface
    /// The connector initialization must be done here
    ///

    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError>;
    
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------


struct VideoInterface {
    actions: Box<dyn VideoActions>
}
type AmVideoInterface = Arc<Mutex<VideoInterface>>;

impl VideoInterface {
    fn new(actions: Box<dyn VideoActions>) -> VideoInterface {
        return VideoInterface {
            actions: actions
        }
    }
    fn new_am(actions: Box<dyn VideoActions>) -> AmVideoInterface {
        return Arc::new(Mutex::new( VideoInterface::new(actions) ));
    }
}


// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

struct VideoStates {
    video_interface: Arc<Mutex<VideoInterface>>
}

///
/// Send video stream on the broker frame by frame
async fn send_video(interface: AmInterface) {
    

    // Init camera object 
    // first camera found in the list
    let index = CameraIndex::Index(0); 

    // request the absolute highest resolution CameraFormat that can be decoded to RGB.
    // let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    // closest of 30fps, 1280x720
    let format = CameraFormat::new_from( 1280, 720, FrameFormat::MJPEG, 30);
    let requested: RequestedFormat<'_> = RequestedFormat::new::<RgbFormat>(RequestedFormatType::Closest(format));


    // try to get the first camera found (if any camera return a error)
    let result_camera = Camera::new(index, requested);
    
    match result_camera {
        Ok(mut camera) => {
            // Send video
            loop {  
                // Get next frame
                let frame = camera.frame().unwrap();
                let frame_value = frame.buffer().to_vec();

                // TO DO : Encode to h264 
                
                // Change frame value and send it on the broker
                interface.lock().await.update_attribute_with_bytes("frame", &frame_value);
                interface.lock().await.publish_all_attributes().await;
            }
        },
        Err(e) => {
            interface.lock().await.log_warn(
                format!("Failed to find camera {}", e)
            );
        }
    }

    // TO DO : Here we should ask to user which camera he wants to use 

    
}

#[async_trait]
impl interface::fsm::States for VideoStates {

    /// Just wait for an fsm event for the connection
    ///
    async fn connecting(&self, interface: &AmInterface)
    {
        interface::basic::wait_for_fsm_event(interface).await;
    }

    /// Initialize the interface
    ///
    async fn initializating(&self, interface: &AmInterface)
    {

        let mut video_itf = self.video_interface.lock().await;

        // Custom initialization slot
        video_itf.actions.initializating(&interface).await.unwrap();

        // Register attributes
        interface.lock().await.register_attribute(RawAttribute::new_boxed("frame", true));
  
        // Send video stream on broker
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        let interface_cloned = interface.clone();

        std::thread::spawn(move || {
            rt.block_on(async {
                let local_set = tokio::task::LocalSet::new();
                local_set.run_until(async {
                    let handle = tokio::task::spawn_local(send_video(interface_cloned));
                    handle.await.unwrap();
                }).await;
            });
        });


        // Notify the end of the initialization
        interface.lock().await.set_event_init_done();
    }

    async fn running(&self, interface: &AmInterface)
    {
        interface::basic::wait_for_fsm_event(interface).await;
    }

    async fn warning(&self, _interface: &AmInterface)
    {
        println!("cleaning");
    }
    async fn cleaning(&self, _interface: &AmInterface)
    {
        println!("cleaning");
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

const ID_STATE: subscription::Id = 0;

struct VideoSubscriber {
    // video_interface: Arc<Mutex<VideoInterface>>
}

impl VideoSubscriber {

}

#[async_trait]
impl interface::subscriber::Subscriber for VideoSubscriber {

    /// Get the list of attributes names
    ///
    async fn attributes_names(&self) -> Vec<(subscription::Id, String)> {
        return vec![
            (ID_STATE, "frame".to_string())
        ];
    }




    /// Process a message
    ///
    async fn process(&self, interface: &AmInterface, msg: &subscription::Message) -> PlatformFunctionResult {
        // Common processing
        interface::basic::process(&interface, msg).await;

        match msg {
            subscription::Message::Mqtt(msg) => {
                match msg.id() {
                subscription::ID_PZA_CMDS_SET => {
                    // interface.lock().await.publish_info().await;

                    // only when running state

                    // println!("VideoSubscriber::process: {:?}", msg.topic());
                    // println!("VideoSubscriber::process: {:?}", msg.payload());

                    // let payload = msg.payload();
                    // let oo = serde_json::from_slice::<Value>(payload).unwrap();
                    // let o = oo.as_object().unwrap();


                    // for (attribute_name, fields) in o.iter() {
                    //     for field in fields.as_object().unwrap().iter() {
                    //         if attribute_name == "frame" {
                    //             // self.process_frame_value(&interface, attribute_name, field.0).await;
                    //         }
                    //     }
                    // }
                    // interface.lock().await.publish_all_attributes().await;


                },
                    _ => {
                        // not managed by the common level
                    }
                }
            }
            _ => {
                // not managed by the common level
            }
        }

        Ok(())
    }


}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Build the meta interface for a Voxpower Channel
///
pub fn build<A: Into<String>>(
    name: A,
    actions: Box<dyn VideoActions>
) -> InterfaceBuilder {

    let c = VideoInterface::new_am(actions);

    return InterfaceBuilder::new(
        name,
        "video",
        "0.0",
        Box::new(VideoStates{video_interface: c.clone()}),
        Box::new(VideoSubscriber{})
    );
}

