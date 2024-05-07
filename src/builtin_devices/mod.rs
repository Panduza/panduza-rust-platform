use crate::device::Factory as DeviceFactory;

mod korad;
mod panduza;

pub fn import_plugin_producers(factory: &mut DeviceFactory)
{
    panduza::import_plugin_producers(factory);  
}
