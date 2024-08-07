mod dio;

use panduza_platform_core::Producer;

// Export the producers of the plugin
//
pub fn plugin_producers() -> Vec<Box<dyn Producer>> {
    let mut producers: Vec<Box<dyn Producer>> = vec![];
    producers.push(dio::producer::PiochaDio::new());
    return producers;
}
