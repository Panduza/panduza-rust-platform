
use std::collections::{HashMap, LinkedList};

use crate::platform::{self, TaskPoolLoader};


mod device;
mod factory;
mod manager;

pub mod traits;

pub type Factory = factory::Factory;
pub type Manager = manager::Manager;
pub type AmManager = manager::AmManager;
pub type Device = device::Device;



