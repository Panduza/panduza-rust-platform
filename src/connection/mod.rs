mod task;
mod manager;
mod connection;

pub type Manager = manager::Manager;
pub type AmManager = manager::AmManager;

pub type Connection = connection::Connection;
pub type AmConnection = connection::AmConnection;

/// Thread safe connection object
pub type ThreadSafeConnection = std::sync::Arc<
                                    tokio::sync::Mutex<
                                        Connection
                                    >
                                >;
