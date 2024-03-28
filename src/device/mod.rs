use std::mem::zeroed;
use std::sync::Arc;

use std::collections::{HashMap, LinkedList};

// use tokio::{task::yield_now, time::{sleep, Duration}};

use crate::connection::AmLinkConnectionManager;
use crate::{builtin_devices, platform_error, subscription};
use crate::interface::AmInterface;

use crate::connection::AmConnection;
use crate::connection::LinkInterfaceHandle;

use serde_json;
use tokio::task::JoinSet;
use tokio::sync::Mutex;

use crate::platform::{self, TaskPoolLoader};
use crate::platform::PlatformError;

use self::traits::DeviceActions;


mod device;
mod factory;
mod manager;

pub mod traits;

pub type Factory = factory::Factory;
pub type Manager = manager::Manager;
pub type AmManager = manager::AmManager;


/// Defines the policy for using the 2 connections (default & operational)
///
#[derive(Clone)]
pub enum ConnectionUsagePolicy {
    /// the device must use both connections if possible
    UseBoth,
    /// the device must use only the default connection
    UseDefaultOnly,
    /// the device must use only the operational connection
    UseOperationalOnly
}



