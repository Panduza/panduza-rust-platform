mod dio;

/// Import the producers of the plugin
/// 
pub fn import_plugin_producers(factory: &mut panduza_core::device::Factory)
{
    factory.add_producer("picoha.dio", dio::DeviceProducer::new_boxed());
}
