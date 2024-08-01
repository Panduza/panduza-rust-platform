use panduza_core::device::Factory as DeviceFactory;

//mod cobolt;
//mod hameg;
//mod korad;
//mod oxxius;
//mod thorlab;
mod panduza;

pub fn import_plugin_producers(factory: &mut DeviceFactory)
{
    //cobolt::import_plugin_producers(factory);
    //hameg::import_plugin_producers(factory);
    //korad::import_plugin_producers(factory);
    //oxxius::import_plugin_producers(factory);
    //thorlab::import_plugin_producers(factory);
    panduza::import_plugin_producers(factory);
}
