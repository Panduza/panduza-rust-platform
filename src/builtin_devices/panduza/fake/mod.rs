use crate::device::Factory as DeviceFactory;

mod power_supply;
mod laser;
mod thermometer;


pub fn import_plugin_producers(factory: &mut DeviceFactory)
{


    factory.add_producer("panduza.fake_power_supply", Box::new(power_supply::DeviceProducer{}));
    factory.add_producer("panduza.fake_laser", Box::new(laser::DeviceProducer{}));
    factory.add_producer("panduza.fake_thermometer", Box::new(thermometer::DeviceProducer{}));

}

