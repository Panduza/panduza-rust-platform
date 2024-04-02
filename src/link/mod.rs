use std::sync::Arc;
use tokio::sync::Mutex;

mod manager;
mod interface_handle;
mod connection_handle;

pub type Manager = manager::Manager;
pub type AmManager = Arc<Mutex<Manager>>;

pub type InterfaceHandle = interface_handle::InterfaceHandle;
pub type ConnectionHandle = connection_handle::ConnectionHandle;
