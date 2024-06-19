use panduza_core::device::Factory as DeviceFactory;

mod ka3005;

pub fn import_plugin_producers(factory: &mut DeviceFactory)
{
    factory.add_producer("korad.ka3005", Box::new(ka3005::DeviceProducer{}));
    factory.add_hunter(Box::new(ka3005::DeviceHunter{}));
}

