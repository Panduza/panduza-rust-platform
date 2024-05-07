use crate::device::Factory as DeviceFactory;

mod ka3005;

pub fn import_plugin_producers(factory: &mut DeviceFactory)
{
    factory.add_producer("korad.ka3005", Box::new(ka3005::DeviceProducer{}));
}

