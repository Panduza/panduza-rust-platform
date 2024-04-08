use std::fmt::Display;

use async_trait::async_trait;
use bitflags::bitflags;
use crate::interface::AmInterface;

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

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            State::Connecting => write!(f, "Connecting"),
            State::Initializating => write!(f, "init"),
            State::Running => write!(f, "run"),
            State::Error => write!(f, "err"),
        }
    }
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

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

#[async_trait]
pub trait States : Send {

    async fn connecting(&self, interface: &AmInterface);
    async fn initializating(&self, interface: &AmInterface);
    async fn running(&self, interface: &AmInterface);
    async fn error(&self, interface: &AmInterface);

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
    interface: AmInterface,

    /// State Implementations
    states: Box<dyn States>,
}

impl Fsm {

    ///
    /// 
    pub fn new(interface: AmInterface, states: Box<dyn States>) -> Fsm {
        Fsm {
            interface: interface,
            states: states,
        }
    }

    ///
    ///
    pub async fn run_once(&mut self) {

        // Get state but do not keep the lock
        let state = self.interface.lock().await.current_state().clone();

        // Debug log
        self.interface.lock().await.log_debug(
            format!("Run State \"{:?}\"", state)
        );

        // Perform state task
        match state {
            State::Connecting => {
                // Execute state
                self.states.connecting(&self.interface).await;

                // Manage transitions
                let evs = self.interface.lock().await.events().clone();

                // If connection up, go to running state
                if evs.contains(Events::CONNECTION_UP) && !evs.contains(Events::ERROR) {
                    self.interface.lock().await.move_to_state(State::Initializating);
                }
            },
            State::Initializating => {
                // Execute state
                self.states.initializating(&self.interface).await;

                // Manage transitions
                let evs = self.interface.lock().await.events().clone();

                // If initialization ok, go to running state
                if evs.contains(Events::INIT_DONE) && !evs.contains(Events::ERROR) {
                    self.interface.lock().await.move_to_state(State::Running);
                }
                // If error, go to error state
                else if evs.contains(Events::ERROR) {
                    self.interface.lock().await.move_to_state(State::Error);
                }
            },
            State::Running => {
                // Execute state
                self.states.running(&self.interface).await;

                // Manage transitions
                let evs = self.interface.lock().await.events().clone();

                // If error, go to error state
                if evs.contains(Events::ERROR) {
                    self.interface.lock().await.move_to_state(State::Error);
                }
                // // If connection down, go to connecting state
                // else if evs.contains(Events::CONNECTION_DOWN) {
                //     self.interface.lock().await.move_to_state(State::Connecting);
                // }
            },
            State::Error => {
                // Execute state
                self.states.error(&self.interface).await;
            }
        }

        // Clear events for next run
        self.interface.lock().await.clear_events();

    }

}

