// use super::file::system_file_path;

/// This object is responsible of the connection information
/// 
/// It must manage the data but also the file used to store them
/// 
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Info {
    // Path of the file
    pub file_path: String,

    // broker info
    pub host_addr: String,
    pub host_port: u16,
    pub host_retry: u32,

    // credential
    
    // Platform info
    pub platform_name: String,
}

impl Info {

    // / Create a new Info object with default values
    // /
    // pub fn default() -> Self {
    //     Self {
    //         file_path: system_file_path().to_str().unwrap().to_string(),
    //         host_addr: "localhost".to_string(),
    //         host_port: 1883,
    //         host_retry: 1,
    //         platform_name: "panduza_platform".to_string()
    //     }
    // }

}
