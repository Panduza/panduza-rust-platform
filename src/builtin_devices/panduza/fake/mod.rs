use crate::device::Factory as DeviceFactory;

mod power_supply;


pub fn import_plugin_producers(factory: &mut DeviceFactory)
{


    factory.add_producer("panduza.fake_power_supply", Box::new(power_supply::DeviceProducer{}));

}

