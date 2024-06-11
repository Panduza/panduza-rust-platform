use async_trait::async_trait;
// use std::sync::Arc;

use crate::platform::PlatformError;
use crate::meta::video;
use crate::interface::AmInterface;
use crate::interface::builder::Builder as InterfaceBuilder;

use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};
use nokhwa::Camera;
use nokhwa_core::pixel_format::RgbFormat;

/// Video Data
/// 
struct VideoActions {
    // camera: Arc<tokio::sync::Mutex<Camera>>,
    // camera: Option<Arc<tokio::sync::Mutex<Camera>>>,
    //camera: Camera,
    // camera: Box<Camera>,
    // camera: <dyn CaptureBackendTrait + Sync>,
    frame_value: Vec<u8>,
    enable_value: bool
}

#[async_trait]
impl video::VideoActions for VideoActions {


    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {

        println!("Searching camera \n");

        // // Init camera object 
        // // first camera found in the list
        // let index = CameraIndex::Index(0); 
        // // request the absolute highest resolution CameraFormat that can be decoded to RGB.
        // let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
        // // make the camera
        // // let mut camera = Camera::new(index, requested).unwrap();

        // self.camera = Some(Arc::new(tokio::sync::Mutex::new(Camera::new(index, requested).unwrap())));

        // // println!("Camera found \n");
        
        // // _interface.lock().await.log_info(
        // //     format!("Find camera")
        // // );
        // // self.camera = self.camera_choice(0);
        return Ok(());
    }

    /// Read the enable value
    /// 
    async fn read_enable_value(&mut self, interface: &AmInterface) -> Result<bool, PlatformError> {
        interface.lock().await.log_info(
            format!("Video - read_enable_value: {}", self.enable_value)
        );
        return Ok(self.enable_value);
    }

    /// Write the enable value
    /// 
    async fn write_enable_value(&mut self, interface: &AmInterface, v: bool) {
        interface.lock().await.log_info(
            format!("Video - write_enable_value: {}", self.enable_value)
        );
        self.enable_value = v;
    }

    /// Read the frame value
    /// 
    async fn read_frame_value(&mut self, interface: &AmInterface) -> Result<&Vec<u8>, PlatformError> {


        interface.lock().await.log_info(
            format!("Video - read_frame_value: received")
        );

        // Here read a frame

        // Init camera object 
        // first camera found in the list
        let index = CameraIndex::Index(0); 
        // request the absolute highest resolution CameraFormat that can be decoded to RGB.
        let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
        // make the camera
        // let mut camera = Camera::new(index, requested).unwrap();

        let mut camera = Camera::new(index, requested).unwrap();

        // println!("Camera found \n");
        
        // _interface.lock().await.log_info(
        //     format!("Find camera")
        // );
        // self.camera = self.camera_choice(0);

        // let frame = camera.lock().await.frame().unwrap();
        let frame = camera.frame().unwrap();
        self.frame_value = frame.buffer().to_vec();
        // let frame_value = frame.buffer();
        return Ok(&(self.frame_value));

        // match &camera_result {
        //     Ok(camera) => {
        //         // let frame = camera.lock().await.frame().unwrap();
        //         let frame = camera.lock().await.frame().unwrap();
        //         self.frame_value = frame.buffer().to_vec();
        //         // let frame_value = frame.buffer();
        //         return Ok(&(self.frame_value));
        //     },
        //     None => {
        //         // No need to maj the variable 
        //         return Ok(&(self.frame_value));
        //     }
        // }
    }
}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A
) -> InterfaceBuilder {

    let vec: Vec<u8> = Vec::new();

    return video::build(
        name, 
        Box::new(VideoActions {
            // camera: None,
            frame_value: vec,
            enable_value: false
        })
    );
}

