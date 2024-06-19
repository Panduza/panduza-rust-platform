use std::io::{self, Read};
use crate::FunctionResult;
use crate::platform::services::Services;

use crate::platform::connection_info::ErrorType;
use crate::platform::connection_info::import_file;
use crate::platform::connection_info::system_file_path;

use crate::__platform_error_result;

// ------------------------------------------------------------------------------------------------

/// Perform the connection info loading process
///
pub async fn execute_connection_info_loading_process(services: &mut Services)
    -> FunctionResult
{
    // Get the system file path
    let file_path = system_file_path();

    // PLATF_00003_00 - display path
    println!("** Connection info file path: {:?}", file_path);

    // Try to import connection info from the system file
    match import_file(file_path).await {
        Ok(ci) => {
            services.set_connection_info(ci);

            // PLATF_00003_00 - display info
            // println!("** Connection info file path: {:?}", file_path);

            Ok(())
        },
        Err(e) => {
            match e.err_type {
                ErrorType::FileDoesNotExist => {
                    return ask_user_about_default_connection_info_creation(services)
                },
                _ => {
                    __platform_error_result!("unmanaged error")
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
    -> FunctionResult
{
    // Warning message
    println!("===========================================================");
    println!("!");
    println!("!");
    println!("!");
    println!("No configuration file found ! ({})", system_file_path().to_str().unwrap());
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
        __platform_error_result!(err_message)
    }
}

