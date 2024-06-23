mod register_map;

/// Import the producers of the plugin
/// 
pub fn import_plugin_producers(factory: &mut panduza_core::device::Factory)
{
    factory.add_producer("fake.register_map", register_map::DeviceProducer::new_boxed());
}
