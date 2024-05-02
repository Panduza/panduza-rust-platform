use async_trait::async_trait;
use crate::platform::PlatformError;
use crate::meta::relay;
use crate::interface::AmInterface;
use crate::interface::builder::Builder as InterfaceBuilder;

use crate::connector::serial::tty;



/// Voxpower Inhibiter Channel Data
/// 
struct VoxpowerInhibiterActions {
    state_open: String,
}

#[async_trait]
impl relay::RelayActions for VoxpowerInhibiterActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError> {
        // let self.channel = channel;
        
        // tty::Get(name)

        return Ok(());
    }

    /// Configuration of the interface
    /// 
    async fn config(&mut self, interface: &AmInterface) -> Result<(), PlatformError> {
        return Ok(());
    }

    /// Read the enable value
    /// 
    async fn read_state_open(&mut self, interface: &AmInterface) -> Result<bool, PlatformError> {
        interface.lock().await.log_info(
            format!("VoxpowerInhibiter - read_state_open {}", self.state_open)
        );
        if self.state_open == "H" {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    /// Write the enable value
    async fn write_state_open(&mut self, interface: &AmInterface, v: bool) {
        if v {
            let command = "I\n"; //format!("I{}\n", self.channel);
            self.state_open = command.to_string();
            interface.lock().await.log_info(
                format!("VoxpowerInhibiter - inhibit channel: {}", self.state_open)
            );
        } else {
            let command = "E\n"; //format!("E{}\n", self.channel);
            self.state_open = command.to_string();
            interface.lock().await.log_info(
                format!("VoxpowerInhibiter - enable channel: {}", self.state_open)
            );
        }
    }
}


/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A,
) -> InterfaceBuilder {

    return relay::build(
        name,
        Box::new(VoxpowerInhibiterActions {
            state_open: "L".to_string(),
        })
    )
}