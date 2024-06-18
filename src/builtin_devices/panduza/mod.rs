use panduza_core::device::Factory as DeviceFactory;

mod test;
mod fake;
mod server;
mod voxpower_inhibiter;
mod video;



pub fn import_plugin_producers(factory: &mut DeviceFactory)
{


    factory.add_producer("panduza.server", Box::new(server::DeviceProducer{}));
    factory.add_producer("panduza.test", Box::new(test::DeviceProducer{}));

    factory.add_producer("panduza.voxpower_inhibiter", Box::new(voxpower_inhibiter::DeviceProducer{}));
    factory.add_producer("panduza.video", Box::new(video::DeviceProducer{}));



    fake::import_plugin_producers(factory);
}

