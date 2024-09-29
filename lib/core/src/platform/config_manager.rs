use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::platform::config_utils;

static DEFAULT_FILENAME: &str = "platform.json";

#[derive(Default, Deserialize, Serialize, Debug)]
struct ConfigJson {
    broker: BrokerInfo,
    platform: Option<PlatformInfo>,
    credentials: Option<CredentialsInfo>,
    services: Option<ServicesInfo>,
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct BrokerInfo {
    pub addr: String,
    pub port: u16,
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct PlatformInfo {
    pub name: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct CredentialsInfo {
    pub user: Option<String>,
    pub pass: Option<String>,
}
#[derive(Default, Deserialize, Serialize, Debug)]
pub struct ServicesInfo {
    pub retry_delay: Option<u64>,
    pub enable_plbd: Option<bool>,
}

#[derive(Default)]
pub struct ConfigManager {
    info: ConfigJson,
}

impl ConfigManager {
    pub fn new() -> ConfigManager {
        Default::default()
    }

    pub fn load_config(&mut self) -> std::io::Result<()> {
        let dir = config_utils::get_default_config_dir()?;
        let filepath = Path::new(dir).join(DEFAULT_FILENAME);

        if filepath.exists() == false {
            self.create_default_file(&filepath)?;
        }

        let file = File::open(&filepath)?;
        self.info = serde_json::from_reader(&file)?;

        Ok(())
    }

    fn create_default_file(&self, filepath: &Path) -> std::io::Result<()> {
        let payload: String;

        let mut file = File::create(&filepath)?;

        let config = ConfigJson {
            broker: BrokerInfo {
                addr: "localhost".to_string(),
                port: 1883,
            },
            platform: None,
            credentials: None,
            services: None,
        };

        payload = serde_json::to_string_pretty(&config)?;

        write!(file, "{}", &payload);
        file.flush()?;

        Ok(())
    }

    pub fn get_broker_info(&self) -> &BrokerInfo {
        &self.info.broker
    }

    pub fn get_platform_info(&self) -> Option<&PlatformInfo> {
        self.info.platform.as_ref()
    }

    pub fn get_credentials_info(&self) -> Option<&CredentialsInfo> {
        self.info.credentials.as_ref()
    }

    pub fn get_services_info(&self) -> Option<&ServicesInfo> {
        self.info.services.as_ref()
    }
}
