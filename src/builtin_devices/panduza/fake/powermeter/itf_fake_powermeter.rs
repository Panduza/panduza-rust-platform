use async_trait::async_trait;

use crate::platform::PlatformError;
use crate::meta::powermeter;
use crate::interface::AmInterface;
use crate::interface::builder::Builder as InterfaceBuilder;


/// Fake Powermeter Channel Data
/// 
struct FakePowermeterActions {
    power_value: f64,
}

#[async_trait]
impl powermeter::PowermeterActions for FakePowermeterActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {
        return Ok(());
    }

    /// Read the power value
    /// 
    async fn read_power_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {
        interface.lock().await.log_info(
            format!("FakePowermeter - read_power_value: {}", self.power_value)
        );
        return Ok(self.power_value);
    }
}



/// Interface to emulate a Thermometer Channel
/// 
pub fn build<A: Into<String>>(
    name: A
) -> InterfaceBuilder {

    return powermeter::build(
        name, 
        powermeter::PowermeterParams {
            power_decimals: 3
        },
        Box::new(FakePowermeterActions {
            power_value: 0.0,
            
        })
    )
}

