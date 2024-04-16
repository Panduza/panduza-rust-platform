use crate::device::Factory as DeviceFactory;

mod fake;
mod server;



pub fn import_plugin_producers(factory: &mut DeviceFactory)
{


    factory.add_producer("panduza.server", Box::new(server::DeviceProducer{}));

    fake::import_plugin_producers(factory);
}

