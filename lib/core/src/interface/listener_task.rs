
use crate::{TaskResult, __platform_error_result};
use super::listener::Listener;

///
///  
/// * `listener` - move the listener into the task 
/// 
pub async fn listener_task(mut listener: Listener) -> TaskResult {

    loop {
        if let Err(_) = listener.run_once().await {
            return __platform_error_result!(
                format!("Interface {:?} Listen Task Error", "TODO")
            );
        }
    }

    Ok(())
}
