use async_trait::async_trait;

use panduza_core::Error as PlatformError;
use panduza_core::meta::thermometer;
use panduza_core::interface::AmInterface;
use panduza_core::interface::builder::Builder as InterfaceBuilder;


/// Fake Thermometer Channel Data
/// 
struct FakeThermometerActions {
    measure_value: f64,
}

#[async_trait]
impl thermometer::ThermometerActions for FakeThermometerActions {
    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {
        return Ok(());
    }

    /// Read the measure value
    /// 
    async fn read_measure_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {
        interface.lock().await.log_info(
            format!("FakeThermometer - read_measure_value: {}", self.measure_value)
        );
        return Ok(self.measure_value);
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
            measure_decimals: 3
        },
        Box::new(FakeThermometerActions {
            measure_value: 0.0,
            
        })
    )
}

