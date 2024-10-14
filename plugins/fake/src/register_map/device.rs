use async_trait::async_trait;
use panduza_platform_core::{
    spawn_on_command, AttOnlyMsgAtt, BidirMsgAtt, Device, DeviceLogger, DeviceOperations, Error,
    MemoryCommandCodec, MemoryCommandMode, NumberCodec, TaskResult,
};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;

///
///
static mut COUNTER: u16 = 0;
///
/// This device is a simulation of a register map that you can access through commands
///
pub struct RegisterMapDevice {
    logger: Option<DeviceLogger>,
    array: Arc<Vec<AttOnlyMsgAtt<NumberCodec>>>,
}

impl RegisterMapDevice {
    ///
    /// Constructor
    ///
    pub fn new() -> RegisterMapDevice {
        RegisterMapDevice {
            logger: None,
            array: Arc::new(Vec::new()),
        }
    }

    ///
    /// Triggered when a new command is received
    ///
    async fn on_command_action(
        logger: DeviceLogger,
        array: Arc<Vec<AttOnlyMsgAtt<NumberCodec>>>,
        mut attr_command: BidirMsgAtt<MemoryCommandCodec>,
    ) -> TaskResult {
        while let Some(command) = attr_command.pop_cmd().await {
            logger.debug(format!("New command {:?}", command));

            match command.mode {
                MemoryCommandMode::Read => {
                    let idx = command.address;
                    unsafe {
                        COUNTER += 1;
                        array[idx as usize].set(COUNTER).await?;
                    }
                }
                MemoryCommandMode::Write => {}
                _ => {}
            }
        }

        Ok(())
    }

    ///
    /// Register map can be updated through memory command
    ///
    async fn create_memory_command_attribute(&mut self, mut device: Device) {
        //
        // Create the attribute
        let attr_command = device
            .create_attribute("command")
            .message()
            .with_bidir_access()
            .finish_with_codec::<MemoryCommandCodec>()
            .await;

        //
        // Execute action on each command received
        let logger = self.logger.as_ref().unwrap().clone();
        let array = self.array.clone();
        spawn_on_command!(
            device,
            attr_command,
            Self::on_command_action(logger.clone(), array.clone(), attr_command.clone())
        );
    }

    ///
    ///
    ///
    async fn create_registers(&mut self, mut device: Device) {
        //
        // Get the logger
        self.logger = Some(device.logger.clone());

        //
        // Register interface
        let mut interface = device.create_interface("registers").finish().await;

        //
        // Create 20 register
        let mut array = Vec::new();
        for n in 0..20 {
            let a = interface
                .create_attribute(format!("{}", n))
                .message()
                .with_att_only_access()
                .finish_with_codec::<NumberCodec>()
                .await;
            a.set(2).await.unwrap();
            array.push(a);
        }
        self.array = Arc::new(array);
    }
}

#[async_trait]
impl DeviceOperations for RegisterMapDevice {
    ///
    /// Mount the device
    ///
    async fn mount(&mut self, device: Device) -> Result<(), Error> {
        //
        // First create registers because command will need them
        self.create_registers(device.clone()).await;
        //
        // Create command
        self.create_memory_command_attribute(device.clone()).await;
        Ok(())
    }

    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, _: Device) {
        sleep(Duration::from_secs(5)).await;
    }
}
