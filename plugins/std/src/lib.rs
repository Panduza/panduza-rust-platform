mod serial_port;

/// Import the producers of the plugin
/// 
pub fn import_plugin_producers(factory: &mut panduza_core::device::Factory)
{
    factory.add_producer("std.serial_port", Box::new(serial_port::DeviceProducer{}));
}
