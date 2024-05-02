use crate::device::Factory as DeviceFactory;

mod fake;
mod server;
mod voxpower_inhibiter;



pub fn import_plugin_producers(factory: &mut DeviceFactory)
{


    factory.add_producer("panduza.server", Box::new(server::DeviceProducer{}));
    factory.add_producer("panduza.voxpower_inhibiter", Box::new(voxpower_inhibiter::DeviceProducer{}));

    fake::import_plugin_producers(factory);
}

