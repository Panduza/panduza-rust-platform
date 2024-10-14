use panduza_platform_core::ProductionOrder;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct DeviceTree {
    ///
    ///
    ///
    pub devices: Vec<ProductionOrder>,
}

// impl DeviceTree {
//     ///
//     ///
//     ///
//     pub fn new() -> DeviceTree {
//         Default::default()
//     }

//     // fn create_default_file(&self, filepath: &Path) -> std::io::Result<()> {
//     //     // let config = DeviceTree::default();
//     //     // let mut file = File::create(filepath)?;
//     //     // let payload = serde_json::to_string_pretty(&config)?;

//     //     // write!(file, "{}", &payload)?;
//     //     // file.flush()?;

//     //     Ok(())
//     // }
// }
