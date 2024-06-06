use crate::device::Factory as DeviceFactory;

mod hm7044;

pub fn import_plugin_producers(factory: &mut DeviceFactory)
{
    factory.add_producer("hameg.hm7044", Box::new(hm7044::DeviceProducer{}));
    factory.add_hunter(Box::new(hm7044::DeviceHunter{}));
}

