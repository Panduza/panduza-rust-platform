use async_trait::async_trait;
use bitflags::bitflags;
use crate::interface::core::AmCore;

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// States of the main Interface FSM
/// 
#[derive(Clone, Debug)]
pub enum State {
    Connecting,
    Initializating,
    Running,
    Error
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Events: u32 {
        const NO_EVENT                  = 0b00000000;
        const CONNECTION_UP             = 0b00000001;
        const CONNECTION_DOWN           = 0b00000010;
        const INIT_DONE                 = 0b00000100;
        const ERROR                     = 0b10000000;
    }
}

impl Events {
    pub fn set_connection_up(&mut self) {
        self.insert(Events::CONNECTION_UP);
    }
    pub fn set_connection_down(&mut self) {
        self.insert(Events::CONNECTION_DOWN);
    }
    pub fn set_init_done(&mut self) {
        self.insert(Events::INIT_DONE);
    }
    pub fn set_state_error(&mut self) {
        self.insert(Events::ERROR);
    }
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

#[async_trait]
pub trait States : Send {

    async fn connecting(&self, core: &AmCore);
    async fn initializating(&self, core: &AmCore);
    async fn running(&self, core: &AmCore);
    async fn error(&self, core: &AmCore);

}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Interface finite state machine
///
pub struct Fsm {
    /// Shared state data
    core: AmCore,

    /// State Implementations
    states: Box<dyn States>,
}

impl Fsm {

    ///
    /// 
    pub fn new(core: AmCore, states: Box<dyn States>) -> Fsm {
        Fsm {
            core: core,
            states: states,
        }
    }

    ///
    ///
    pub async fn run_once(&mut self) {
        
        // Get state but do not keep the lock
        let state = self.data.lock().await.current_state().clone();

        // Perform state task
        match state {
            State::Connecting => {
                // Execute state
                self.states.connecting(&self.data).await;
                
                // Manage transitions
                let evs = self.data.lock().await.events().clone();

                // If connection up, go to running state
                if evs.contains(Events::CONNECTION_UP) && !evs.contains(Events::ERROR) {
                    self.data.lock().await.move_to_state(State::Initializating);
                }
            },
            State::Initializating => {
                // Execute state
                self.states.initializating(&self.data).await;

                // Manage transitions
                let evs = self.data.lock().await.events().clone();

                // If initialization ok, go to running state
                if evs.contains(Events::INIT_DONE) && !evs.contains(Events::ERROR) {
                    self.data.lock().await.move_to_state(State::Running);
                }
                // If error, go to error state
                else if evs.contains(Events::ERROR) {
                    self.data.lock().await.move_to_state(State::Error);
                }
            },
            State::Running => {
                // Execute state
                self.states.running(&self.data).await;

                // Manage transitions
                let evs = self.data.lock().await.events().clone();

                // If error, go to error state
                if evs.contains(Events::ERROR) {
                    self.data.lock().await.move_to_state(State::Error);
                }
                // // If connection down, go to connecting state
                // else if evs.contains(Events::CONNECTION_DOWN) {
                //     self.data.lock().await.move_to_state(State::Connecting);
                // }
            },
            State::Error => {
                // Execute state
                self.states.error(&self.data).await;
            }
        }

        // Clear events for next run
        self.data.lock().await.clear_events();

    }

}

