use panduza_platform_core::ProductionOrder;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::env::consts::OS;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct DeviceTree {
    devices: Vec<ProductionOrder>,
}

impl DeviceTree {
    ///
    ///
    ///
    pub fn new() -> DeviceTree {
        Default::default()
    }

    // fn create_default_file(&self, filepath: &Path) -> std::io::Result<()> {
    //     // let config = DeviceTree::default();
    //     // let mut file = File::create(filepath)?;
    //     // let payload = serde_json::to_string_pretty(&config)?;

    //     // write!(file, "{}", &payload)?;
    //     // file.flush()?;

    //     Ok(())
    // }
}
