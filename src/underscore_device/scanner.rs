// pub mod data;

use panduza_platform_core::{Device, Error};

///
/// Mount the scanner attribute
///
pub async fn mount(mut instance: Device) -> Result<(), Error> {
    //
    // Create the attribute
    let class_scanner = instance.create_interface("scanner").finish();

    //
    //
    Ok(())
}
