use panduza_platform_core::Device;

///
/// Mount the store attribute
///
/// json with all the possible device that can be created + scanned instances found on the server
/// {
///     "manuf.model" : {
///         "description" : "text"
///         "settings": {}  -> description of the possible settings for the driver
///         "instances": {} -> json that can be copy/past in the tree.json
///     }
/// }
///
pub async fn mount(mut device: Device) -> Result<(), Error> {
    //
    // Create the attribute
    let att_store = device
        .create_attribute("store")
        .with_ro()
        .finish_as_json()
        .await?;
}
