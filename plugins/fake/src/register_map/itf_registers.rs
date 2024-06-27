use async_trait::async_trait;
use panduza_core::Error as PlatformError;
use panduza_core::meta::registers;
use panduza_core::interface::ThreadSafeInterface;
use panduza_core::interface::builder::Builder as InterfaceBuilder;
use serde_json::Error;


///
/// 
struct RegisterMapActions {

    pub fake_values: Vec<u64>,
}

#[async_trait]
impl registers::RegistersActions for RegisterMapActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _:&ThreadSafeInterface) -> Result<(), PlatformError> {
        return Ok(());
    }
    
    async fn read(&mut self, interface: &ThreadSafeInterface, index:usize, size:usize) -> Result<Vec<u64>, String>
    {
        if let Some(sub_vec) = self.fake_values.get(index..index+size) {
            // Étape 4: Utiliser sub_vec ici
            println!("Sous-vecteur: {:?}", sub_vec);
            Ok(sub_vec.to_vec())
        } else {
            // Gérer l'erreur si la plage est invalide
            println!("Plage invalide");
            Err("invalid".to_string())
        }
    }

    async fn write(&mut self, interface: &ThreadSafeInterface, index:usize, v: &Vec<u64>)
    {
        println!("RegisterMapActions - write: {:?}", v);
    }


}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A
) -> InterfaceBuilder {

    let fake_size = 10;

    return registers::build(
        name, 
        registers::RegistersSettings {
            base_address: 0x0000,
            register_size: registers::RegisterSize::_32bits,
            number_of_register: fake_size
        }, 
        Box::new(RegisterMapActions {
            fake_values: vec![0; fake_size]
            // enable_value: false,
            // voltage_value: 0.0,
            // current_value: 0.0,
            // time_lock_duration: Some(tokio::time::Duration::from_millis(100)),
        })
    )
}

