use panduza_core::device::Factory as DeviceFactory;

mod hm7044;

pub fn import_plugin_producers(factory: &mut DeviceFactory)
{
    factory.add_producer("hameg.hm7044", Box::new(hm7044::DeviceProducer{}));
    // No hunter instanciated, as the HM7044 could be controlled through any USB to serial probe.
}

