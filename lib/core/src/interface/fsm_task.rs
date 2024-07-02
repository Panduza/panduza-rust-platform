use super::fsm::Fsm;
use crate::TaskResult;

/// Task code that runs the interface FSM
///  
/// * `fsm` - move the fsm into the task 
/// 
pub async fn fsm_task(mut fsm: Fsm) -> TaskResult {

    loop {
        fsm.run_once().await;
    }
    

    Ok(())
}

