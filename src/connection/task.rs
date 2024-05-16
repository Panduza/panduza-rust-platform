use super::ThreadSafeConnection;
use crate::platform::TaskResult;

/// Run the connection
/// 
/// \todo: pass as parameter the connection object inside of all its components (connection will be clonable)
/// 
async fn task(connection: ThreadSafeConnection) -> TaskResult
{

    Ok(())
}

