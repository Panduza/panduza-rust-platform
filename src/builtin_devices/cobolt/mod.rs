use crate::device::Factory as DeviceFactory;

mod s0501;

pub fn import_plugin_producers(factory: &mut DeviceFactory)
{
    factory.add_producer("cobolt.s0501", Box::new(s0501::DeviceProducer{}));
    factory.add_hunter(Box::new(s0501::DeviceHunter{}));
}

