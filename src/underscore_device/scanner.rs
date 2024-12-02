pub mod data;

use data::ScannerDriver;
use panduza_platform_core::log_debug;
use panduza_platform_core::{spawn_on_command, BooleanAttServer, Error, Instance, InstanceLogger};

///
/// Mount the scanner attribute
///
/// scanner -> interface to control a scan session
///      - running boolean
///      - total_scan number
///      - joined_scan number
///      - instances json
///
pub async fn mount(mut instance: Instance, driver: ScannerDriver) -> Result<(), Error> {
    //
    // Create the attribute
    let mut class_scanner = instance.create_class("scanner").finish();

    let att_running = class_scanner
        .create_attribute("running")
        .with_rw()
        .finish_as_boolean()
        .await?;
    att_running.set(false).await?;

    //
    // Execute action on each command received
    let logger_2 = instance.logger.clone();
    let att_running_2 = att_running.clone();
    spawn_on_command!(
        instance,
        att_running_2,
        on_running_command(logger_2.clone(), att_running_2.clone(), driver.clone())
    );

    //
    //
    Ok(())
}

///
///
///
async fn on_running_command(
    logger: InstanceLogger,
    mut att_running: BooleanAttServer,
    mut driver: ScannerDriver,
) -> Result<(), Error> {
    while let Some(command) = att_running.pop_cmd().await {
        //
        // Log
        log_debug!(logger, "Scanner run command received '{:?}'", command);

        //
        // object partagé => trigger notify

        driver.request_scanning_start().await;
        att_running.set(true).await?;

        // -> thread dans la platform
        // un thread attend le notify
        // when ok => platform request for scan
        // flag running => true
    }
    Ok(())
}
