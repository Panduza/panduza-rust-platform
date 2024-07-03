use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio::time::Duration;

use async_trait::async_trait;

use panduza_core::{interface, Error as PlatformError};
use panduza_core::meta::digital_input;
use panduza_core::interface::ThreadSafeInterface;
use panduza_core::interface::builder::Builder as InterfaceBuilder;
use serde_json::Error;

use futures::FutureExt;

///
/// 
struct InterfaceActions {

    // pub fake_values: Arc<Mutex<Vec<u64>>>,
}

#[async_trait]
impl digital_input::MetaActions for InterfaceActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, interface :&ThreadSafeInterface) -> Result<(), PlatformError> {

        // let interface_locked = interface.lock().await;
        // let mut loader = interface_locked.platform_services.lock().await.task_loader.clone();

        // let values = self.fake_values.clone();
        // loader.load( async move {
        //     loop {
        //         sleep(Duration::from_millis(1000)).await;
        //         values.lock().await[1] += 1;
        //     }
        //     // Ok(())
        // }.boxed()).unwrap();


        return Ok(());
    }
    
    
    async fn read(&mut self, interface: &ThreadSafeInterface) -> Result<u8, String>
    {
        // let values = self.fake_values.lock().await;
        return Ok(0 as u8);
    }



    // async fn read(&mut self, interface: &ThreadSafeInterface, index:usize, size:usize) -> Result<Vec<u64>, String>
    // {
    //     if let Some(sub_vec) = self.fake_values.lock().await.get(index..index+size) {
    //         // Étape 4: Utiliser sub_vec ici
    //         println!("Sous-vecteur: {:?}", sub_vec);
    //         Ok(sub_vec.to_vec())
    //     } else {
    //         // Gérer l'erreur si la plage est invalide
    //         println!("Plage invalide");
    //         Err("invalid".to_string())
    //     }
    // }

    // async fn write(&mut self, interface: &ThreadSafeInterface, index:usize, v: &Vec<u64>)
    // {
    //     self.fake_values.lock().await.splice(index..index+v.len(), v.iter().cloned());
    //     println!("InterfaceActions - write: {:?}", v);
    // }


}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A
) -> InterfaceBuilder {

    let fake_size = 10;

    return digital_input::build(
        name,
        Box::new(InterfaceActions {
            // fake_values: Arc::new( Mutex::new( vec![0; fake_size] ))
        })
    )
}

