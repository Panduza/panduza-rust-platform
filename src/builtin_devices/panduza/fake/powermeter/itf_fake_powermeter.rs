use async_trait::async_trait;

use crate::platform::PlatformError;
use crate::meta::powermeter;
use crate::interface::AmInterface;
use crate::interface::builder::Builder as InterfaceBuilder;


/// Fake Powermeter Channel Data
/// 
struct FakePowermeterActions {
    measure_value: f64,
}

#[async_trait]
impl powermeter::PowermeterActions for FakePowermeterActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {
        return Ok(());
    }

    /// Read the measure value
    /// 
    async fn read_measure_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {
        interface.lock().await.log_info(
            format!("FakePowermeter - read_measure_value: {}", self.measure_value)
        );
        self.measure_value += 0.001;
        return Ok(self.measure_value);
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
            measure_decimals: 3
        },
        Box::new(FakePowermeterActions {
            measure_value: 0.0,
            
        })
    )
}

