use crate::device::Factory as DeviceFactory;

mod cobolt;
mod korad;
mod panduza;
mod thorlab;

pub fn import_plugin_producers(factory: &mut DeviceFactory)
{
    cobolt::import_plugin_producers(factory);
    korad::import_plugin_producers(factory);
    panduza::import_plugin_producers(factory);
    thorlab::import_plugin_producers(factory);
}
