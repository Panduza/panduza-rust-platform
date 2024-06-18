use async_trait::async_trait;
// use std::sync::Arc;

use crate::platform::PlatformError;
use crate::meta::video;
use crate::interface::AmInterface;
use crate::interface::builder::Builder as InterfaceBuilder;

/// Video Data
/// 
struct VideoActions {
    // frame_value: Vec<u8>
}

#[async_trait]
impl video::VideoActions for VideoActions {


    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {

        return Ok(());
    }
}



/// Interface to make video
/// 
pub fn build<A: Into<String>>(
    name: A
) -> InterfaceBuilder {

    // let vec: Vec<u8> = Vec::new();

    return video::build(
        name, 
        Box::new(VideoActions {
            // camera: None,
            // frame_value: vec
        })
    );
}

