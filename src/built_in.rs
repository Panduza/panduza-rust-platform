use panduza_platform_core::Producer;
use panduza_platform_core::Scanner;

// Export the producers of the plugin
//
pub fn plugin_producers() -> Vec<Box<dyn Producer>> {
    let mut producers: Vec<Box<dyn Producer>> = vec![];
    producers.extend(pza_plugin_vi::plugin_producers());
    producers.extend(pza_plugin_korad::plugin_producers());
    // producers.extend(pza_plugin_hameg::plugin_producers());

    producers.extend(pza_plugin_picoha::plugin_producers());
    producers.extend(pza_plugin_hantek::plugin_producers());

    return producers;
}

//
//
pub fn plugin_scanners() -> Vec<Box<dyn Scanner>> {
    let mut scanners: Vec<Box<dyn Scanner>> = vec![];
    scanners.extend(pza_plugin_vi::plugin_scanners());
    scanners.extend(pza_plugin_korad::plugin_scanners());
    // scanners.extend(pza_plugin_hameg::plugin_scanners());

    scanners.extend(pza_plugin_picoha::plugin_scanners());
    scanners.extend(pza_plugin_hantek::plugin_scanners());

    return scanners;
}
