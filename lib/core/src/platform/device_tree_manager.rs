use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::env::consts::OS;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::platform::config_utils;

static DEFAULT_FILENAME: &str = "tree.json";
static DEFAULT_DIR_UNIX: &str = "/etc/panduza";
static DEFAULT_DIR_WINDOWS: &str = r"C:\ProgramData\Panduza";

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct DeviceTreeJson {
    devices: Vec<DeviceInfo>,
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct DeviceInfo {
    pub r#ref: String,
    pub name: String,
    pub settings: Option<serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct DeviceTreeManager {
    info: DeviceTreeJson,
}

impl DeviceTreeManager {
    pub fn new() -> DeviceTreeManager {
        Default::default()
    }

    pub fn load_device_tree(&mut self) -> std::io::Result<()> {
        let dir = config_utils::get_default_config_dir()?;
        let filepath = Path::new(dir).join(DEFAULT_FILENAME);

        if filepath.exists() == false {
            self.create_default_file(&filepath)?;
        }

        let file = File::open(filepath)?;
        self.info = serde_json::from_reader(&file)?;

        Ok(())
    }

    fn create_default_file(&self, filepath: &Path) -> std::io::Result<()> {
        let config = DeviceTreeJson::default();
        let mut file = File::create(filepath)?;
        let payload = serde_json::to_string_pretty(&config)?;

        write!(file, "{}", &payload)?;
        file.flush()?;

        Ok(())
    }

    pub fn get_tree(&self) -> &Vec<DeviceInfo> {
        &self.info.devices
    }
}
