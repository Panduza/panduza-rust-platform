use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::env::consts::OS;
use std::fs::File;
use std::io::Write;
use std::path::Path;

static DEFAULT_FILENAME: &str = "platform.json";
static DEFAULT_DIR_UNIX: &str = "/etc/panduza";
// TODO: add the default path for windows
static DEFAULT_DIR_WINDOWS: &str = "TODO";

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

    pub fn load_config(&mut self) -> Result<(), std::io::Error> {
        let dir = self.get_default_config_dir()?;
        let filename = format!("{}/{}", dir, DEFAULT_FILENAME);
        let mut file: File;

        if Path::new(&filename).exists() == false {
            file = self.create_default_file(&filename)?;
        }

        file = File::open(&filename)?;
        self.info = serde_json::from_reader(&file)?;

        Ok(())
    }

    fn create_default_file(&self, filename: &str) -> Result<File, std::io::Error> {
        let mut file = File::create(&filename)?;
        let mut config: ConfigJson = Default::default();
        let payload: String;

        config.broker.addr = "localhost".to_string();
        config.broker.port = 1883;

        payload = serde_json::to_string_pretty(&config)?;

        file.write_all(&payload.as_bytes())?;

        Ok(file)
    }

    fn get_default_config_dir(&self) -> Result<&str, std::io::Error> {
        let dir: &str;

        match OS {
            "linux" => dir = DEFAULT_DIR_UNIX,
            "windows" => dir = DEFAULT_DIR_WINDOWS,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unsupported OS",
                ))
            }
        }

        Ok(dir)
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
