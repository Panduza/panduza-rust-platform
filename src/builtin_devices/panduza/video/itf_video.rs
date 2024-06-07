use async_trait::async_trait;
use std::sync::Arc;

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
    camera: <dyn CaptureBackendTrait + Sync>,
    frame_value: &[u8],
    enable_value: bool
}

#[async_trait]
impl video::VideoActions for VideoActions {

    /// Camera init 
    /// Need to make the user choose the camera he wants to use
    // async fn camera_choice(&mut self, interface: &AmInterface, index_camera: u32) -> Option<Arc<tokio::sync::Mutex<Camera>>> {
    //     interface.lock().await;
    //     // Init camera object 
    //     // first camera in system
    //     let index = CameraIndex::Index(index_camera); 
    //     // request the absolute highest resolution CameraFormat that can be decoded to RGB.
    //     let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
    //     // make the camera
    //     // let mut camera = Camera::new(index, requested).unwrap();

    //     return Some(
    //         Arc::new(tokio::sync::Mutex::new(
    //             Camera::new(index, requested).unwrap()))
    //     );
        
    // }

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {
        // _interface.lock().await.log_info(
        //     format!("Find camera")
        // );
        // self.camera = self.camera_choice(0);
        return Ok(());
    }

    /// Read the enable value
    /// 
    async fn read_enable_value(&mut self, interface: &AmInterface) -> Result<bool, PlatformError> {
        self.camera.
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

    /// Read the mode value
    /// 
    async fn read_frame_value(&mut self, interface: &AmInterface) -> Result<&[u8], PlatformError> {
        interface.lock().await.log_info(
            format!("Video - read_frame_value: received")
        );

        // Here read a frame

        let mut frame_val = String::new();
        swap(&mut frame_val, &mut self.frame_value);
        return Ok(frame_val);
    }
}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A
) -> InterfaceBuilder {

    return video::build(
        name, 
        Box::new(VideoActions {
            camera: None
            frame_value: 0,
            enable_value: false
        })
    );
}

