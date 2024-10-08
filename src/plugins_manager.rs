use panduza_platform_core::Error;
use panduza_platform_core::PlatformLogger;
use panduza_platform_core::Plugin;
use panduza_platform_core::ProductionOrder;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

///
/// Gather all the objects required to make the plugin work
///
pub struct PluginHandler {
    ///
    /// Binary object loaded inside process
    object: libloading::Library,
    ///
    /// C interface of the plugin
    interface: Plugin,
}

impl PluginHandler {
    ///
    /// Load a plugin from a file
    ///
    pub fn from_filename(filename: PathBuf) -> Result<PluginHandler, Error> {
        unsafe {
            //
            // Load library object
            let object = libloading::Library::new(filename.clone()).map_err(|e| {
                Error::PluginError(format!(
                    "Unable to load plugin [{:?}] - ({:?})",
                    filename, e
                ))
            })?;

            //
            // Get plugin interface from entry point
            let plugin_entry_point: libloading::Symbol<extern "C" fn() -> Plugin> =
                object.get(b"plugin_entry_point").map_err(|e| {
                    Error::PluginError(format!(
                        "Unable to load plugin_entry_point [{:?}] - ({:?})",
                        filename, e
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
pub struct PluginsManager {
    /// logger
    logger: PlatformLogger,

    ///
    /// Plugin handlers
    ///
    handlers: Vec<PluginHandler>,
}

impl PluginsManager {
    ///
    /// Create a new object
    ///
    pub fn new() -> Self {
        Self {
            logger: PlatformLogger::new(),

            handlers: Vec::new(),
        }
    }

    ///
    ///
    pub fn plugins_system_paths(&mut self) -> Vec<PathBuf> {
        let mut res = Vec::new();
        // res.push(value);
        // a coté du binaire
        // si windows c:/
        let path = env::current_exe().unwrap();
        let parent = path.parent().unwrap();
        let ppp = parent.join("plugins");
        // println!("The current directory is {}", ppp.display()); // cd/plugins

        res.push(ppp);

        // main and alternate

        let windows_path = PathBuf::from(dirs::public_dir().unwrap())
            .join("panduza")
            .join("plugins");
        // println!("The current directory is {}", windows_path.display()); // cd/plugins

        res.push(windows_path);

        return res;
    }

    ///
    ///
    pub fn load_system_plugins(&mut self) -> Result<u32, Error> {
        let mut count = 0;

        for path in self.plugins_system_paths() {
            // User information
            self.logger
                .info(format!("Search Plugins in ({})", path.display()));

            // Ensure the path is a directory
            if path.is_dir() {
                // Iterate over directory entries
                for entry in fs::read_dir(path).unwrap() {
                    let entry = entry.unwrap();
                    let path = entry.path();

                    // Check if the entry is a file and has a DLL extension
                    if path.is_file() && path.extension() == Some(OsStr::new("dll")) {
                        // Print or process the DLL file path
                        println!("Found DLL file: {}", path.display());

                        self.register_plugin(path)?;
                        count += 1;
                        // Add the DLL file path to a list or perform other actions as needed
                    }
                }
            }
        }
        Ok(count)
    }

    ///
    /// To register a new plugin
    ///
    pub fn register_plugin(&mut self, filename: PathBuf) -> Result<(), Error> {
        //
        // Append the plugin
        self.handlers.push(PluginHandler::from_filename(filename)?);
        Ok(())
    }

    ///
    ///
    ///
    pub fn produce(order: ProductionOrder) {
        // find the good plugin
        // produce the device
    }
}
