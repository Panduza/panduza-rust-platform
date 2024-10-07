use panduza_platform_core::Error;

pub enum Services {
    Boot,
    LoadPlugins,
    StartBroker,
}

// run actions when request arrives
//
pub struct ServicesTask {}

/// Task code that runs the interface Listener
///
/// move the listener into the task
///
pub async fn service_task() -> Result<(), Error> {
    // loop {

    // }

    println!("service_task");
    Ok(())
}
