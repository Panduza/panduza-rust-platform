pub mod data;

use data::SharedStore;
use panduza_platform_core::{Error, Instance};

///
/// Mount the store attribute
///
/// json with all the possible driver that can be instanciated + scanned instances found on the server
/// {
///     "manuf.model" : {
///         "description" : "text"
///         "settings": {}  -> description of the possible settings for the driver
///     }
/// }
///
pub async fn mount(mut instance: Instance, store: SharedStore) -> Result<(), Error> {
    //
    // Create the attribute
    let att_store = instance
        .create_attribute("store")
        .with_ro()
        .finish_as_json()
        .await?;

    //
    //
    let value = store.into_json_value().await?;
    att_store.set(value).await?;

    //
    //
    let store_has_changed = store.change_notifier.clone();

    //
    //
    instance
        .spawn(async move {
            //
            loop {
                //
                // Wait for store change
                store_has_changed.notified().await;

                let value = store.into_json_value().await?;
                att_store.set(value).await?;
            }
        })
        .await;

    //
    //
    Ok(())
}
