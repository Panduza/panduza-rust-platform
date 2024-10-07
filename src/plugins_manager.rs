use panduza_platform_core::Error;
use panduza_platform_core::Plugin;

///
///
///
struct PluginHandler {
    ///
    /// Binary object loaded inside process
    object: libloading::Library,
    ///
    /// C interface of the plugin
    interface: Plugin,
}

impl PluginHandler {
    ///
    ///
    ///
    pub fn from_filename<A: Into<String>>(filename: A) -> Result<PluginHandler, Error> {
        //
        // Set filename type
        let filename_string: String = filename.into();
        unsafe {
            //
            // Load library object
            let object = libloading::Library::new(filename_string.clone()).map_err(|e| {
                Error::PluginError(format!(
                    "Unable to load plugin [{:?}] - ({:?})",
                    filename_string, e
                ))
            })?;

            //
            // Get plugin interface from entry point
            let plugin_entry_point: libloading::Symbol<extern "C" fn() -> Plugin> =
                object.get(b"plugin_entry_point").map_err(|e| {
                    Error::PluginError(format!(
                        "Unable to load plugin_entry_point [{:?}] - ({:?})",
                        filename_string, e
                    ))
                })?;
            let interface = plugin_entry_point();

            //
            // Compose the handler
            // Object must be kept alive as long as the interface live
            return Ok(PluginHandler {
                object: object,
                interface: interface,
            });
        }
    }
}

///
///
///
struct PluginsManager {
    ///
    /// Plugin handlers
    ///
    handlers: Vec<PluginHandler>,
}

impl PluginsManager {
    //
}
