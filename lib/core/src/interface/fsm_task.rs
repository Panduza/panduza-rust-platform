



use crate::TaskResult;
use super::fsm::Fsm;

///
///  
/// * `fsm` - move the fsm into the task 
/// 
pub async fn fsm_task(mut fsm: Fsm) -> TaskResult {

    loop {
        fsm.run_once().await;
    }
    

    Ok(())
}









