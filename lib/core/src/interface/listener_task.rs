
use crate::TaskResult;
use super::listener::Listener;

///
///  
/// * `listener` - move the listener into the task 
/// 
pub async fn listener_task(listener: Listener) -> TaskResult {

// loop {
//     if let Err(_) = listener.lock().await.run_once().await {
//         return __platform_error_result!(
//             format!("Interface {:?} Listen Task Error", interface_name)
//         );
//     }
// }

    Ok(())
}
