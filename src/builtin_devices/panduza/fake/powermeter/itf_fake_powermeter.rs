use async_trait::async_trait;

use crate::platform::PlatformError;
use crate::meta::powermeter;
use crate::interface::AmInterface;
use crate::interface::builder::Builder as InterfaceBuilder;


/// Fake Powermeter Channel Data
/// 
struct FakePowermeterActions {
    mesure_value: f64,
}

#[async_trait]
impl powermeter::PowermeterActions for FakePowermeterActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {
        return Ok(());
    }

    /// Read the mesure value
    /// 
    async fn read_mesure_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {
        interface.lock().await.log_info(
            format!("FakePowermeter - read_mesure_value: {}", self.mesure_value)
        );
        return Ok(self.mesure_value);
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
            mesure_decimals: 3
        },
        Box::new(FakePowermeterActions {
            mesure_value: 0.0,
            
        })
    )
}

