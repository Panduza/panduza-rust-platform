


// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Subscription ID
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConnectionStatusMessage {

    /// Connection status (True if connected, False if disconnected)
    pub connected: bool,

}

impl ConnectionStatusMessage {

    /// Create a new connection status message
    pub fn new(connected: bool) -> ConnectionStatusMessage {
        return ConnectionStatusMessage {
            connected: connected
        }
    }

}
