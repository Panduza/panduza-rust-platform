use crate::{device::Factory as DeviceFactory, meta::thermometer};

mod power_supply;
mod thermometer;


pub fn import_plugin_producers(factory: &mut DeviceFactory)
{


    factory.add_producer("panduza.fake_power_supply", Box::new(power_supply::DeviceProducer{}));
    factory.add_producer("panduza.fake_thermometer", Box::new(thermometer::DeviceProducer{}));

}

