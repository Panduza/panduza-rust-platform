use std::io::{self, Read};
use crate::platform::{connection_info::{self, ConnectionInfo}, services::Services};

// ------------------------------------------------------------------------------------------------

/// Perform the connection info loading process
///
pub async fn execute_connection_info_loading_process(services: &mut Services)
    -> Result<(),  &'static str >
{
    // Try to import connection info from the user file
    match ConnectionInfo::build_from_file().await {
        // Set the connection info if build from file is ok
        Ok(ci) => {
            services.set_connection_info(ci);
            Ok(())
        },
        // Else Manage errors and unperfect situations
        Err(e) => {
            match e.type_() {
                connection_info::CiErrorType::FileDoesNotExist => {
                    return ask_user_about_default_connection_info_creation(services)
                },
                _ => {
                    Err("unmanaged error")
                }
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

/// Ask the user if he wants to create a default connection info.
/// Stop the platform in case it does not work or the user does not want.
///
fn ask_user_about_default_connection_info_creation(services: &mut Services)
    -> Result<(),  &'static str >
{
    // Warning message
    println!("===========================================================");
    println!("!");
    println!("!");
    println!("!");
    println!("No configuration file found ! ({})", ConnectionInfo::system_file_path().to_str().unwrap());
    println!("Do you want to create one with default settings ? [N/y]");

    // Get input from user
    let mut input = [0; 1];
    io::stdin().read(&mut input).unwrap();
    let char = input[0] as char;

    // Check if user answer Yes
    // If so generate a default connection info
    if char == 'y' || char == 'Y' {
        services.generate_default_connection_info()
            .map_err(|e| {
                println!("");
                println!("Failed to create default connection info: {}", e.to_string());
                println!("?");
                println!("Check that you have the permission for creating the file");
                println!("?");
                println!("");
            })
            .expect("Failed to create default connection info");
        println!("Default connection info created !");
        println!("!");
        println!("!");
        println!("!");
        println!("===========================================================");
        Ok(())
    }
    // Other answers are considered as No
    // and an error must be returned
    else {
        let err_message: &'static str = "No connection info set ! stopping the platform...";
        println!("{}", err_message);
        println!("!");
        println!("!");
        println!("!");
        println!("===========================================================");
        services.trigger_stop();
        Err(err_message)
    }
}

