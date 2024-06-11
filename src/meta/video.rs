use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::Mutex;

use crate::attribute::{JsonAttribute, RawAttribute};
use crate::interface::AmInterface;
use crate::platform::PlatformError;
use crate::{interface, subscription};
use crate::interface::builder::Builder as InterfaceBuilder;

use crate::platform::FunctionResult as PlatformFunctionResult;

#[async_trait]
pub trait VideoActions: Send + Sync {

    /// Initialize the interface
    /// The connector initialization must be done here
    ///

    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError>;
    
    async fn read_frame_value(&mut self, interface: &AmInterface) -> Result<&Vec<u8>, PlatformError>;

    async fn read_enable_value(&mut self, interface: &AmInterface) -> Result<bool, PlatformError>;

    async fn write_enable_value(&mut self, interface: &AmInterface, v: bool);
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------


// pub struct StateAttribute {
//     attr: JsonAttribute,
// }


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
        // self.video_interface.lock().await.actions.initializating(&interface).await.unwrap();

        // Register attributes
        interface.lock().await.register_attribute(RawAttribute::new_boxed("frame", true));
        interface.lock().await.register_attribute(JsonAttribute::new_boxed("enable", true));

        // Init frame
        // let state_value = self.video_interface.lock().await.actions.read_state_open(&interface).await.unwrap();
        // interface.lock().await.update_attribute_with_f64("frame", "value", state_value).unwrap();
        
        // update attribute with byte type
        let frame_value = video_itf.actions.read_frame_value(&interface).await.unwrap();
        interface.lock().await.update_attribute_with_bytes("frame", frame_value);

        // Init enable
        let enable_value = video_itf.actions.read_enable_value(&interface).await.unwrap();
        interface.lock().await.update_attribute_with_bool("enable", "value", enable_value).unwrap();

        // Publish all attributes for start
        interface.lock().await.publish_all_attributes().await;

        // Notify the end of the initialization
        interface.lock().await.set_event_init_done();
    }

    async fn running(&self, interface: &AmInterface)
    {
        let mut video_itf = self.video_interface.lock().await;
        
        // Send video

        loop {
            // println!("pif");


            // // Init camera object 
            // // first camera found in the list
            // let index = CameraIndex::Index(0); 
            // // request the absolute highest resolution CameraFormat that can be decoded to RGB.
            // let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
            // // make the camera
            // // let mut camera = Camera::new(index, requested).unwrap();

            // let mut camera = Camera::new(index, requested).unwrap();

            // // println!("Camera found \n");
            
            // // _interface.lock().await.log_info(
            // //     format!("Find camera")
            // // );
            // // self.camera = self.camera_choice(0);

            // // let frame = camera.lock().await.frame().unwrap();
            // let frame = camera.frame().unwrap();
            // self.frame_value = frame.buffer().to_vec();
            // // let frame_value = frame.buffer();

            
            let frame_value = video_itf.actions.read_frame_value(&interface).await.unwrap();
            // println!("paf");
            interface.lock().await.update_attribute_with_bytes("frame", frame_value);

            // println!("argggg");
            
            interface.lock().await.publish_all_attributes().await;
        }

        // interface::basic::wait_for_fsm_event(interface).await;
    }

    async fn error(&self, _interface: &AmInterface)
    {
        println!("error");
    }

    // async fn cleaning(&self, _interface: &AmInterface)
    // {
    //     println!("cleaning");
    // }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

const ID_STATE: subscription::Id = 0;

struct VideoSubscriber {
    video_interface: Arc<Mutex<VideoInterface>>
}

impl VideoSubscriber {

    /// 
    /// 
    #[inline(always)]
    async fn process_enable_value(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str, field_data: &Value) {
        let requested_value = field_data.as_bool().unwrap();
        self.video_interface.lock().await
            .actions.write_enable_value(&interface, requested_value).await;

        let r_value = self.video_interface.lock().await
            .actions.read_enable_value(&interface).await
            .unwrap();

        interface.lock().await
            .update_attribute_with_bool("enable", "value", r_value).unwrap();
    }

    /// 
    /// 
    #[inline(always)]
    async fn process_frame_value(&self, interface: &AmInterface, _attribute_name: &str, _field_name: &str) {
        
        let mut video_interface = self.video_interface.lock().await;

        let r_value = video_interface
            .actions.read_frame_value(&interface).await
            .unwrap();
        
        // update attribute with bytes array

        interface.lock().await
            .update_attribute_with_bytes("frame", r_value)

    }
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

                    println!("VideoSubscriber::process: {:?}", msg.topic());
                    println!("VideoSubscriber::process: {:?}", msg.payload());

                    let payload = msg.payload();
                    let oo = serde_json::from_slice::<Value>(payload).unwrap();
                    let o = oo.as_object().unwrap();


                    for (attribute_name, fields) in o.iter() {
                        for (field_name, field_data) in fields.as_object().unwrap().iter() {
                            if attribute_name == "enable" && field_name == "value" {
                                self.process_enable_value(&interface, attribute_name, field_name, field_data).await;
                            }
                            if attribute_name == "frame" {
                                self.process_frame_value(&interface, attribute_name, field_name).await;
                            }
                        }
                    }
                    interface.lock().await.publish_all_attributes().await;


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
        Box::new(VideoSubscriber{video_interface: c.clone()})
    );
}

