use super::file::system_file_path;

// ---
// Documentation
// https://panduza.github.io/panduza-doc/docs/50_platform/architecture/connection_info/
// ---

/// This object is responsible of the connection information
/// 
/// It must manage the data but also the file used to store them
/// 
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Info {
    // Path of the file
    pub file_path: String,

    // broker info
    pub broker_addr: String,
    pub broker_port: u16,

    // credential
    pub credentials_user: Option<String>,
    pub credentials_pass: Option<String>,

    // Platform info
    pub platform_name: String,

    // Services info
    pub services_retry_delay: u32,
    pub services_enable_plbd: bool,
}

impl Info {

    /// Create a new Info object with default values
    ///
    pub fn default() -> Self {
        Self {
            file_path: system_file_path().to_str().unwrap().to_string(),
            broker_addr: "localhost".to_string(),
            broker_port: 1883,
            credentials_user: None,
            credentials_pass: None,
            platform_name: "panduza_platform".to_string(),
            services_retry_delay: 1,
            services_enable_plbd: false,
        }
    }

}
