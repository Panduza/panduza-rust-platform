mod video_no_compression;

/// Import the producers of the plugin
/// 
pub fn import_plugin_producers(factory: &mut panduza_core::device::Factory)
{
    factory.add_producer("panduza.video", video_no_compression::DeviceProducer::new_boxed());
}
