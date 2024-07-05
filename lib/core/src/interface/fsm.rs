use std::fmt::Display;

use async_trait::async_trait;
use bitflags::bitflags;
use crate::interface::AmInterface;

use crate::TaskResult;
use crate::Error as PlatformError;

use tokio::time::sleep;
use tokio::time::Duration;

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
    Warning,
    Cleaning,
    Stopping,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            State::Connecting => write!(f, "Connecting"),
            State::Initializating => write!(f, "Initializating"),
            State::Running => write!(f, "Running"),
            State::Warning => write!(f, "Warning"),
            State::Cleaning => write!(f, "Cleaning"),
            State::Stopping => write!(f, "Stopping"),
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
        const REBOOT                    = 0b00001000;
        const STOP                      = 0b00010000;
        const CLEANED                   = 0b00100000;
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

    /// Without a broker connection, the interface is useless and must wait for it.
    ///
    async fn connecting(&self, interface: &AmInterface);

    /// The interface is now connected to a broker and need some initialization tasks.
    /// This state must hold the initialization of the connector and the initial atttribute values.
    ///
    async fn initializating(&self, interface: &AmInterface) -> Result<(), PlatformError>;

    /// The interface is now running and can perform its main operationnal state.
    /// 
    async fn running(&self, interface: &AmInterface);

    /// This function must warn the user that something is wrong with the interface.
    /// Then this function must watch for the event that will trigger the reboot of the interface.
    /// 
    async fn warning(&self, interface: &AmInterface);

    // The interface must be able to clean up all resources before being destroyed.
    //
    async fn cleaning(&self, interface: &AmInterface);
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

    /// Task code that runs the interface FSM
    /// 
    pub async fn run_task(mut self) -> TaskResult {

        loop {
            self.run_once().await;
        }
        

        Ok(())
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
                    self.interface.lock().await.move_to_state(State::Warning);
                }
                //
                else if evs.contains(Events::STOP) {
                    self.interface.lock().await.move_to_state(State::Stopping);
                }
            },
            State::Running => {
                // Execute state
                self.states.running(&self.interface).await;

                // Manage transitions
                let evs = self.interface.lock().await.events().clone();

                // If error, go to error state
                if evs.contains(Events::ERROR) {
                    self.interface.lock().await.move_to_state(State::Warning);
                }
                // If connection down, go to connecting state
                else if evs.contains(Events::CONNECTION_DOWN) {
                    self.interface.lock().await.move_to_state(State::Warning);
                }
                //
                else if evs.contains(Events::STOP) {
                    self.interface.lock().await.move_to_state(State::Stopping);
                }
            },
            State::Warning => {
                // Execute state
                // self.states.warning(&self.interface).await;

                // Wait for 5 sec and reboot
                sleep(Duration::from_secs(5)).await;

                // Manage transitions
                let evs = self.interface.lock().await.events().clone();

                if evs.contains(Events::REBOOT) {
                    self.interface.lock().await.move_to_state(State::Cleaning);
                }
            }
            State::Cleaning => {
                // Execute state
                self.states.cleaning(&self.interface).await;

                // Manage transitions
                let evs = self.interface.lock().await.events().clone();

                if evs.contains(Events::CLEANED) {
                    self.interface.lock().await.move_to_state(State::Connecting);
                }
            }
            State::Stopping => {
                // Execute state
                self.states.cleaning(&self.interface).await;
            }
        }

        // Clear events for next run
        self.interface.lock().await.clear_events();

        //
        self.interface.lock().await.publish_all_attributes().await;

    }

}

