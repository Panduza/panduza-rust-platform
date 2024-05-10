use async_trait::async_trait;
use crate::platform::PlatformError;
use crate::meta::bpc;
use crate::interface::AmInterface;
use crate::interface::builder::Builder as InterfaceBuilder;

/// Fake Bench Power Channel Data
/// 
struct FakeBpcActions {
    enable_value: bool,
    voltage_value: f64,
    current_value: f64,
}

#[async_trait]
impl bpc::BpcActions for FakeBpcActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError> {
        return Ok(());
    }

    /// Read the enable value
    /// 
    async fn read_enable_value(&mut self, interface: &AmInterface) -> Result<bool, PlatformError> {
        interface.lock().await.log_info(
            format!("FakeBpc - read_enable_value: {}", self.enable_value)
        );
        return Ok(self.enable_value);
    }

    async fn write_enable_value(&mut self, interface: &AmInterface, v: bool) {
        interface.lock().await.log_info(
            format!("FakeBpc - write_enable_value: {}", self.enable_value)
        );
        self.enable_value = v;
    }

    /// Read the voltage value
    /// 
    async fn read_voltage_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {
        interface.lock().await.log_info(
            format!("FakeBpc - read_voltage_value: {}", self.voltage_value)
        );
        return Ok(self.voltage_value);
    }

    async fn write_voltage_value(&mut self, interface: &AmInterface, v: f64) {
        interface.lock().await.log_info(
            format!("FakeBpc - write_voltage_value: {}", v)
        );
        self.voltage_value = v;
    }
 
    async fn read_current_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {
        interface.lock().await.log_info(
            format!("FakeBpc - read_current_value: {}", self.current_value)
        );
        return Ok(self.current_value);
    }

    async fn write_current_value(&mut self, interface: &AmInterface, v: f64) {
        interface.lock().await.log_info(
            format!("FakeBpc - write_current_value: {}", v)
        );
        self.current_value = v;
    }

}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A
) -> InterfaceBuilder {

    return bpc::build(
        name, 
        bpc::BpcParams {
            voltage_min: 0.0,
            voltage_max: 5.0,
            voltage_decimals: 2,

            current_min: 0.0,
            current_max: 3.0,
            current_decimals: 3,
        }, 
        Box::new(FakeBpcActions {
            enable_value: false,
            voltage_value: 0.0,
            current_value: 0.0,
        })
    )
}

