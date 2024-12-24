use panduza_platform_core::env::system_default_config_dir;
use panduza_platform_core::log_warn;
use panduza_platform_core::Logger;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct BrokerConfig {
    pub addr: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServicesConfig {
    pub enable_plbd: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // Platform info
    pub platform_name: Option<String>,

    // broker info
    pub broker: Option<BrokerConfig>,

    // Services info
    pub services: Option<ServicesConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            platform_name: Some("platform".to_string()),
            broker: Some(BrokerConfig {
                addr: Some("127.0.0.1".to_string()),
                port: Some(1883),
            }),
            services: Some(ServicesConfig {
                enable_plbd: Some(false),
            }),
        }
    }
}

/// Get the platform configuration from the default config file
///
pub fn get_platform_config(logger: Logger) -> Config {
  
    let file_path = system_default_config_dir().unwrap().join("platform.toml");
    let config_content = if file_path.exists() {
        std::fs::read_to_string(&file_path).expect("Failed to read config file")
    } else {
        let default_config = Config::default();
        let toml_content =
            toml::to_string(&default_config).expect("Failed to serialize default config");

      if let Err(e) = std::fs::write(&file_path, &toml_content) {
            log_warn!(logger, "Failed to write default config file: {}", e);
        }

        toml_content
    };
    toml::from_str(&config_content).expect("Failed to parse config file")
}
