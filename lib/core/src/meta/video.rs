use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::time::SystemTime;

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

use openh264::encoder::Encoder;
use openh264::formats::{YUVBuffer, RgbSliceU8};
use jpeg_decoder::Decoder;

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

// Transform frame mjpeg to RGB
fn mjpeg_to_rgb(mjpeg_data: &[u8]) -> image::RgbImage {
    let mut decoder = Decoder::new(mjpeg_data);
    let pixels = decoder.decode().expect("Failed to decode MJPEG frame");
    let metadata = decoder.info().expect("Failed to get MJPEG metadata");
    let width = metadata.width as u32;
    let height = metadata.height as u32;
    let rgb_image: image::RgbImage = image::ImageBuffer::from_raw(width, height, pixels).expect("Failed to create RGB image from raw data");
    rgb_image
}

///
/// Send video stream on the broker frame by frame
async fn send_video(interface: AmInterface) {
    

    // Init camera object 
    // first camera found in the list
    let index = CameraIndex::Index(1); 

    // request the absolute highest resolution CameraFormat that can be decoded to RGB.
    // let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    // closest of 30fps, 1280x720
    // let format = CameraFormat::new_from( 1280, 720, FrameFormat::NV12, 30);
    let format = CameraFormat::new_from( 1280, 720, FrameFormat::MJPEG, 30);
    let requested: RequestedFormat<'_> = RequestedFormat::new::<RgbFormat>(RequestedFormatType::Closest(format));

    // try to get the first camera found (if any camera return a error)
    let result_camera = Camera::new(index, requested);

    // Initialize the encoder  
    let mut encoder = Encoder::new().unwrap();

    let mut sys_time = SystemTime::now();
    let mut frame_transmited = 0;

    match result_camera {
        Ok(mut camera) => {
            if cfg!(windows) {
                 // Send video
                loop {  
                    // every second show the number of frame send 
                    // match SystemTime::now().duration_since(sys_time) {
                    //     Ok(value) => {
                    //         if (value.as_millis() > 1000) {
                    //             println!("frame transmited : {}", frame_transmited);
                    //             sys_time = SystemTime::now();
                    //             frame_transmited = 0;
                    //         }
                    //     },
                    //     Err(e) => {
                    //         println!("error : {}", e);
                    //     }
                    // }

                    // Get next frame (open the stream if it didn't have been done before)
                
                    let frame = camera.frame().unwrap();
                    let frame_value = frame.buffer().to_vec();

                    // Encode to h264 using Mjpeg with cpu

                    // Convert the frame to RGBImage
                    // let rgb_image = image::RgbImage::from_raw(1280, 720, frame.buffer().to_vec()).unwrap();
                    let rgb_image = mjpeg_to_rgb(frame.buffer());

                    let width = rgb_image.width() as usize;
                    let height = rgb_image.height() as usize;
                    let rgb_slice = RgbSliceU8::new(rgb_image.as_raw(), (width, height));
                    
                    // Convert RGB image to YUV
                    // let yuv_buffer = rgb_to_yuv(&rgb_image);
                    let yuv_buffer = YUVBuffer::from_rgb_source(rgb_slice);
                    let encoded_frame: openh264::encoder::EncodedBitStream = encoder.encode(&yuv_buffer).unwrap();


                    // Encode to h264 using NV12 (YUV)
                    
                    // let yuv_buffer = YUVBuffer::from_vec(frame.buffer().to_vec(), 1280, 720);

                    // Encode the frame
                    // let encoded_frame: openh264::encoder::EncodedBitStream = encoder.encode(&yuv_buffer).unwrap();
                    // let frame_bytes: &[u8] = &encoded_frame.to_vec();

                    // let frame_bytes: &[u8] = &frame.buffer().to_vec();

                    // let mut file = File::create("proute.yuv").unwrap();
                    // file.write_all(frame_bytes).unwrap();

                    // TO DO : here get directly the frame encode with camera to h264
                    
                    // return;
                
                    // save as file 

                    // let frame_bytes: &[u8] = &encoded_frame.to_vec();
                    // let mut file = File::create("proute.h264").unwrap();
                    // file.write_all(frame_bytes).unwrap();
                    // return;



                    // Change frame value and send it on the broker
                    interface.lock().await.update_attribute_with_bytes("frame", &encoded_frame.to_vec());
                    interface.lock().await.publish_all_attributes().await;
                    frame_transmited += 1;
                    if (frame_transmited >= 10) {
                        println!("paf");
                        frame_transmited = 0;
                    }
                }
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

    async fn error(&self, _interface: &AmInterface)
    {
        println!("error");
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

