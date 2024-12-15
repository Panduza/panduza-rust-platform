pub mod data;

use data::ScannerDriver;
use panduza_platform_core::{log_debug, Container, JsonAttServer, Logger};
use panduza_platform_core::{
    spawn_loop, spawn_on_command, BooleanAttServer, Error, Instance, InstanceLogger,
};
use serde_json::json;

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
    let mut class_scanner = instance.create_class("scanner").finish().await;

    let att_running = class_scanner
        .create_attribute("running")
        .with_rw()
        .finish_as_boolean()
        .await?;
    att_running.set(false).await?;

    let att_result = class_scanner
        .create_attribute("result")
        .with_ro()
        .finish_as_json()
        .await?;
    att_result.set(json!({})).await?;

    //
    //
    let driver_2 = driver.clone();
    // let logger_3 = instance.logger.clone();
    spawn_loop!("loop => _/scanner/result", instance, {
        driver_2.update_notifier.notified().await;
        let ppp = driver_2.into_json_value().await.unwrap();

        att_result.set(ppp).await?;
    });

    //
    // Execute action on each command received
    let logger_2 = instance.logger.clone();
    let att_running_2 = att_running.clone();
    spawn_on_command!(
        "on_command => _/scanner",
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
    logger: Logger,
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

///
///
///
async fn wait_for_notification(
    logger: InstanceLogger,
    mut att_result: JsonAttServer,
    mut driver: ScannerDriver,
) -> Result<(), Error> {
    // while let Some(command) = att_running.pop_cmd().await {
    //
    // Log
    // log_debug!(logger, "Scanner run command received '{:?}'", command);

    //
    // object partagé => trigger notify

    // driver.request_scanning_start().await;
    // att_running.set(true).await?;

    // -> thread dans la platform
    // un thread attend le notify
    // when ok => platform request for scan
    // flag running => true
    // }
    Ok(())
}
