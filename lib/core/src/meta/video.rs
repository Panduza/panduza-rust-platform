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

// use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType, CameraFormat, FrameFormat};
// use nokhwa::Camera;
// use nokhwa_core::pixel_format::RgbFormat;

// use openh264::encoder::Encoder;
// use openh264::formats::{YUVBuffer, RgbSliceU8};
// use jpeg_decoder::Decoder;

use windows::core::Interface;
use windows::Win32::Media::MediaFoundation::*;
use windows::Win32::System::Com::*;
use windows::Win32::Foundation::*;

use h264_webcam_stream::*;


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


async unsafe fn extract_sample_data(sample: &IMFSample) -> Vec<u8> {
    // Get the buffer count
    let buffer_count = sample.GetBufferCount().unwrap();

    // match hr {
    //     Ok(v) => {
    //         // println!("");
    //     },
    //     Err(e) => {
    //         println!("error getting buffer count from sample");
    //     }
    // }

    // Iterate through each buffer to extract data
    let mut total_data: Vec<u8> = Vec::new();
    for i in 0..buffer_count {
        // let mut buffer: Option<IMFMediaBuffer> = None;
        let mut buffer = sample.GetBufferByIndex(i).unwrap();

        // Lock the buffer to access its data
        let mut buffer_data = std::ptr::null_mut();
        let mut max_length = 0;
        let mut current_length = 0;
        let hr = buffer.Lock(&mut buffer_data, 
            Some(&mut max_length), Some(&mut current_length));
        // if hr != S_OK {
        //     return Err(windows::Error::HRESULT(hr).into());
        // }

        // Convert the locked buffer data into Vec<u8>
        let data_slice = std::slice::from_raw_parts(buffer_data as *const u8, current_length as usize);
        total_data.extend_from_slice(data_slice);

        // Unlock the buffer
        buffer.Unlock();
    }

    return total_data;
}

async unsafe fn capture_video_frames(reader: &IMFSourceReader, interface: AmInterface) {
    loop {
        let mut flags: u32 = 0;
        let mut sample: Option<IMFSample> = None;
        let mut timestamp = 0;
        
        // Read a sample from the video stream
        let hr = reader.ReadSample(
            MF_SOURCE_READER_FIRST_VIDEO_STREAM.0 as u32,
            0,
            None,
            Some(&mut flags),
            Some(&mut timestamp),
            Some(&mut sample),
        );

        // Check HRESULT for success or end of stream (end of stream is never supposed happened)
        match hr {
            Ok(v) => {
                // Process the video sample (e.g., encode, save, or display it)
                if let Some(sample) = sample {
                    // If there's a sample, extract data from it
                    let buffer = extract_sample_data(&sample).await;
                    // println!("Captured a video sample at timestamp: {}", timestamp);
                    // println!("Sample data length: {}", buffer.len());
                    // interface.lock().await.update_attribute_with_bytes("frame", &buffer.to_vec());
                    // interface.lock().await.publish_all_attributes().await;
                    
                } else {
                    // Handle case when sample is None
                    println!("Sample is None");
                    // Vec::new() // or any other handling for empty sample case
                }
                
            },
            Err(e) => {
                println!("Error reading video sample: {:?}", e);
            }
        }
    }
}

async unsafe fn configure_video_capture(interface: AmInterface) {
    // Create attributes for device enumeration
    let mut attributes: Option<IMFAttributes> = None;
    MFCreateAttributes(&mut attributes, 1).unwrap();
    let attributes = attributes.unwrap();
    attributes.SetGUID(&MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE, 
        &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID).unwrap();

    // Enumerate devices
    let mut devices_ptr: *mut Option<IMFActivate> = std::ptr::null_mut();
    let mut count: u32 = 0;
    MFEnumDeviceSources(&attributes, &mut devices_ptr, &mut count).unwrap();

    if count == 0 {
        panic!("No video capture devices found.");
    }

    // Retrieve the first device
    let devices = std::slice::from_raw_parts(devices_ptr, count as usize);
    let device = devices[0].as_ref().unwrap();

    // Activate the first device
    let source: IMFMediaSource = device.ActivateObject().unwrap();

    // Create a source reader from the media source
    let reader: IMFSourceReader = MFCreateSourceReaderFromMediaSource(&source, None).unwrap();

    // Set the media type for the first video stream (usually stream 0)
    let media_type: IMFMediaType = MFCreateMediaType().unwrap();
    media_type.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video).unwrap();
    media_type.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_H264).unwrap();

    // Set other necessary attributes of media type
    // Example: Set frame size and frame rate if required
    // media_type.SetUINT32(&MF_MT_FRAME_SIZE, width | (height << 16))?;
    // media_type.SetUINT32(&MF_MT_FRAME_RATE, numerator | (denominator << 32))?;_FIRST_VIDEO_STREAM.0 as u32, None, &media_type).unwrap();

    // Set the media type on the reader
    let hr = reader.SetCurrentMediaType(MF_SOURCE_READER_FIRST_VIDEO_STREAM.0 as u32, None, &media_type);
    match hr {
        Ok(v) => {
            println!("Success creating camera");
        },
        Err(e) => {
            println!("error creating camera : {:?}", e);
        }
    }

    // Capture video frames
    capture_video_frames(&reader, interface).await;

    // Clean up
    reader.Flush(MF_SOURCE_READER_FIRST_VIDEO_STREAM.0 as u32).unwrap();
    source.Shutdown().unwrap();
}


///
/// Send video stream on the broker frame by frame
async fn send_video(interface: AmInterface) {

    if cfg!(target_os = "windows") {
        // TO DO : Here we should ask to user which camera he wants to use 
        unsafe {
            // Initialize COM
            CoInitializeEx(None, COINIT_MULTITHREADED);

            // Initialize Media Foundation
            let _ = MFStartup(MF_VERSION, MFSTARTUP_LITE).unwrap();

            // Configure the video capture and encoding pipeline
            let _ = configure_video_capture(interface).await;

            // Shutdown Media Foundation
            unsafe { MFShutdown().unwrap() };

            // Uninitialize COM
            CoUninitialize();
        }    
    } else if cfg!(target_os = "linux") {
        // let device_path = Path::new("/dev/video0");
        let max_fps = 60;

        let devices: Vec<_> = h264_webcam_stream::list_devices()
            .into_iter()
            .collect();

        println!("Video devices: {:?}", devices);
    
        // let mut device = h264_webcam_stream::get_device(&device_path).unwrap();
        let mut stream = h264_webcam_stream::stream(devices[0], max_fps).unwrap();
    
        // let mut f = std::fs::File::create("./test.h264")?;
        
        loop {
            let (h264_bytes, _) = stream.next(false).unwrap();
            // Record the h264 video to a file
            // f.write_all(&h264_bytes[..])?;
            interface.lock().await.update_attribute_with_bytes("frame", &h264_bytes);
            interface.lock().await.publish_all_attributes().await;
        }
    }
    
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

