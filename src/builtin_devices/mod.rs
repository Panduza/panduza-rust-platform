use crate::device::Factory as DeviceFactory;

mod korad;
mod cobolt;
mod panduza;

pub fn import_plugin_producers(factory: &mut DeviceFactory)
{
    korad::import_plugin_producers(factory);
    panduza::import_plugin_producers(factory);
}
