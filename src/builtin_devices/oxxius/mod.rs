use panduza_core::device::Factory as DeviceFactory;

mod lbx_488;

pub fn import_plugin_producers(factory: &mut DeviceFactory)
{
    factory.add_producer("oxxius.lbx_488", Box::new(lbx_488::DeviceProducer{}));
    factory.add_hunter(Box::new(lbx_488::DeviceHunter{}));
}

