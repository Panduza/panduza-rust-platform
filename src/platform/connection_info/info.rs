/// This object is responsible of the connection information
/// 
/// It must manage the data but also the file used to store them
/// 
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Info {
    // Path of the file
    file_path: String,

    // broker info
    host_addr: String,
    host_port: u16,
    host_retry: u32,

    // credential
    
    // Platform info
    platform_name: String,
}

impl Info {

    /// Create a new Info object with default values
    ///
    pub fn default() -> Self {
        Self {
            file_path: Info::system_file_path().to_str().unwrap().to_string(),
            host_addr: "localhost".to_string(),
            host_port: 1883,
            host_retry: 1,
            platform_name: "panduza_platform".to_string()
        }
    }





    /// Getter Hostname
    ///
    pub fn host_addr(&self) -> &String {
        &self.host_addr
    }

    ///
    /// 
    pub fn platform_name(&self) -> &String {
        &self.platform_name
    }

    /// Getter Port
    ///
    pub fn host_port(&self) -> u16 {
        self.host_port
    }



}
