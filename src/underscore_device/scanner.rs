// pub mod data;

use panduza_platform_core::{Device, Error};

///
/// Mount the scanner attribute
///
/// scanner -> interface to control a scan session
///      - running boolean
///      - total_scan number
///      - joined_scan number
///      - instances json
///
pub async fn mount(mut instance: Device) -> Result<(), Error> {
    //
    // Create the attribute
    let class_scanner = instance.create_interface("scanner").finish();

    //
    //
    Ok(())
}
