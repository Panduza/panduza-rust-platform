use panduza_core::device::Factory as DeviceFactory;

mod pm100a;

pub fn import_plugin_producers(factory: &mut DeviceFactory)
{
    factory.add_producer("thorlabs.pm100a", Box::new(pm100a::DeviceProducer{}));
    factory.add_hunter(Box::new(pm100a::DeviceHunter{}));
}

