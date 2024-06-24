use async_trait::async_trait;
use panduza_core::Error as PlatformError;
use panduza_core::meta::registers;
use panduza_core::interface::AmInterface;
use panduza_core::interface::builder::Builder as InterfaceBuilder;


///
/// 
struct RegisterMapActions {

}

#[async_trait]
impl registers::RegistersActions for RegisterMapActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {


        // let mut response: &mut [u8] = &mut [0; 1024];
        // let _result = self.connector_tty.write_then_read(
        //     b"*IDN?",
        //     &mut response,
        //     self.time_lock_duration
        // ).await
        //     .map(|c| {
        //         let pp = &response[0..c];
        //         let sss = String::from_utf8(pp.to_vec()).unwrap();
        //         println!("RegistersActions - initializating: {:?}", sss);
        //     });


        return Ok(());
    }
    
    async fn read(&mut self, interface: &AmInterface, index:u32, size:u32) -> Result<String, PlatformError>
    {
        Ok(String::from(""))
    }

    async fn write(&mut self, interface: &AmInterface, index:u32, v: &Vec<u64>)
    {
        println!("RegisterMapActions - write: {:?}", v);
    }


}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A
) -> InterfaceBuilder {

    return registers::build(
        name, 
        registers::RegistersParams {
            base_address: 0x0000,
            register_size: 32,
            number_of_register: 10
        }, 
        Box::new(RegisterMapActions {
            // enable_value: false,
            // voltage_value: 0.0,
            // current_value: 0.0,
            // time_lock_duration: Some(tokio::time::Duration::from_millis(100)),
        })
    )
}

