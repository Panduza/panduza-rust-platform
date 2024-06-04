use async_trait::async_trait;

use crate::platform::PlatformError;
use crate::meta::thermometer;
use crate::interface::AmInterface;
use crate::interface::builder::Builder as InterfaceBuilder;


/// Fake Thermometer Channel Data
/// 
struct FakeThermometerActions {
    temperature_value: f64,
}

#[async_trait]
impl thermometer::ThermometerActions for FakeThermometerActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {
        return Ok(());
    }

    /// Read the temperature value
    /// 
    async fn read_temperature_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {
        interface.lock().await.log_info(
            format!("FakeThermometer - read_temperature_value: {}", self.temperature_value)
        );
        return Ok(self.temperature_value);
    }
}



/// Interface to emulate a Thermometer Channel
/// 
pub fn build<A: Into<String>>(
    name: A
) -> InterfaceBuilder {

    return thermometer::build(
        name, 
        thermometer::ThermometerParams {
            temperature_decimals: 3
        },
        Box::new(FakeThermometerActions {
            temperature_value: 0.0,
            
        })
    )
}

