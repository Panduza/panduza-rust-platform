use panduza_platform_core::env;
use panduza_platform_core::Error;
use panduza_platform_core::Notification;
use panduza_platform_core::PlatformLogger;
use panduza_platform_core::Plugin;
use panduza_platform_core::ProductionOrder;
use panduza_platform_core::Store;
use std::ffi::CStr;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

///
/// Gather all the objects required to make the plugin work
///
pub struct PluginHandler {
    ///
    /// Binary object loaded inside process
    _object: libloading::Library,
    ///
    /// C interface of the plugin
    interface: Plugin,
    ///
    ///
    store: Store,
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
            //
            let store = interface.store_as_obj().unwrap();

            //
            // Compose the handler
            // Object must be kept alive as long as the interface live
            return Ok(PluginHandler {
                _object: object,
                interface: interface,
                store: store,
            });
        }
    }

    ///
    /// Const ref on store
    ///
    pub fn store(&self) -> &Store {
        &self.store
    }

    ///
    /// Produce the device if it can
    ///
    /// Return
    /// - True if the plugin successfuly build the device
    /// - False if it cannot build it
    /// - Error if it can but failed to do it
    ///
    pub fn produce(&self, order: &ProductionOrder) -> Result<bool, Error> {
        unsafe {
            if self.store.contains(&order.dref) {
                let order_as_c_string = order.to_c_string()?;
                let ret = (self.interface.produce)(order_as_c_string.as_c_str().as_ptr());
                println!("==> {}", ret);
                return Ok(true);
            }
        }
        Ok(false)
    }

    ///
    ///
    ///
    pub fn pull_notifications(&self) -> Result<Vec<Notification>, Error> {
        unsafe {
            let notifs_as_ptr = (self.interface.pull_notifications)();

            //
            //
            if notifs_as_ptr.is_null() {
                return Err(Error::InvalidArgument("Null C string pointer".to_string()));
            }
            //
            //
            let c_str = CStr::from_ptr(notifs_as_ptr);
            let str = c_str
                .to_str()
                .map_err(|e| Error::InvalidArgument(format!("Invalid C string: {:?}", e)))?;

            let json: serde_json::Value = serde_json::from_str(str)
                .map_err(|e| Error::InvalidArgument(format!("Invalid JSON: {:?}", e)))?;

            let obj = serde_json::from_value(json.clone()).map_err(|e| {
                Error::InvalidArgument(format!(
                    "Failed to deserialize 'Notification' from JSON string: {:?} {:?}",
                    e, json
                ))
            })?;

            // println!("pulll {:?}", obj);

            Ok(obj)
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
    pub fn load_system_plugins(&mut self) -> Result<u32, Error> {
        let mut count = 0;

        //
        // Get extension of the plugin on the system
        let dyn_lib_ext =
            env::system_dyn_lib_extension().map_err(|e| Error::Generic(format!("{:?}", e)))?;

        for path in env::system_plugins_dir_paths() {
            // User information
            self.logger
                .info(format!("? SEARCH PUGINS in ({})", path.display()));

            // Ensure the path is a directory
            if path.is_dir() {
                // Iterate over directory entries
                for entry in fs::read_dir(path).unwrap() {
                    let entry = entry.unwrap();
                    let path = entry.path();

                    // Check if the entry is a file and has a DLL extension
                    if path.is_file() && path.extension() == Some(OsStr::new(dyn_lib_ext.as_str()))
                    {
                        // Print or process the DLL file path
                        self.logger
                            .info(format!("!  Found PUGIN file: {:?}", path.display()));

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
        let handler = PluginHandler::from_filename(filename)?;

        // Info
        self.logger
            .info(format!("         PRODUCERS : {:?}", handler.store()));

        //
        // Append the plugin
        self.handlers.push(handler);
        Ok(())
    }

    ///
    /// True when a plugin was able to build the order, false else
    ///
    pub fn produce(&mut self, order: &ProductionOrder) -> Result<bool, Error> {
        for ph in (&self.handlers).into_iter() {
            match ph.produce(order)? {
                true => return Ok(true),
                false => {}
            }
        }

        //
        // Log failure
        self.logger
            .info(format!("No plugin found to manage this order"));
        Ok(false)
    }

    ///
    ///
    ///
    pub fn pull_notifications(&self) -> Result<Vec<Notification>, Error> {
        //
        //
        let mut results: Vec<Notification> = Vec::new();

        for ph in (&self.handlers).into_iter() {
            results.extend(ph.pull_notifications()?);
        }

        Ok(results)
    }

    ///
    /// Merge all the stores from plugins into a single one
    ///
    pub fn merge_stores(&mut self) -> Store {
        let mut store = Store::default();
        for ph in (&self.handlers).into_iter() {
            store.extend_by_copy(&ph.store);
        }
        store
    }
}
