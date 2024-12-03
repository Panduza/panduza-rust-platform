pub mod att;
mod devices;
pub mod pack;
pub mod pack_inner;
pub mod scanner;
pub mod store;
pub mod structure;
pub mod topic;

use async_trait::async_trait;
use pack::InfoPack;
use panduza_platform_core::{DriverOperations, Error, Instance};
use scanner::data::ScannerDriver;
use std::time::Duration;
use store::data::SharedStore;
use tokio::time::sleep;
pub use topic::Topic;

///
/// Main device of the platform
/// Provides the informations about the platform
///
pub struct UnderscoreDevice {
    ///
    ///
    ///
    pack: InfoPack,

    ///
    ///
    ///
    store: SharedStore,

    scanner_driver: ScannerDriver,
}

impl UnderscoreDevice {
    ///
    /// Constructor
    ///
    pub fn new(store: SharedStore, scanner_driver: ScannerDriver) -> (UnderscoreDevice, InfoPack) {
        let pack = InfoPack::new();

        let device = UnderscoreDevice {
            pack: pack.clone(),
            store: store,
            scanner_driver: scanner_driver,
        };

        (device, pack)
    }
}

#[async_trait]
impl DriverOperations for UnderscoreDevice {
    ///
    ///
    ///
    async fn mount(&mut self, instance: Instance) -> Result<(), Error> {
        //
        // Mount the store
        store::mount(instance.clone(), self.store.clone()).await?;

        //
        //
        scanner::mount(instance.clone(), self.scanner_driver.clone()).await?;

        //
        // Mount devices
        devices::mount(instance.clone(), self.pack.clone()).await?;

        //
        // Mount structure
        structure::mount(instance.clone(), self.pack.clone()).await?;

        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut _device: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
