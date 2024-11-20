pub mod data;

use panduza_platform_core::{Device, Error};

///
/// Mount the store attribute
///
/// json with all the possible driver that can be instanciated + scanned instances found on the server
/// {
///     "manuf.model" : {
///         "description" : "text"
///         "settings": {}  -> description of the possible settings for the driver
///         "instances": {} -> json that can be copy/past in the tree.json
///     }
/// }
///
pub async fn mount(mut instance: Device) -> Result<(), Error> {
    //
    // Create the attribute
    let att_store = instance
        .create_attribute("store")
        .with_ro()
        .finish_as_json()
        .await?;

    //
    //
    Ok(())
}
