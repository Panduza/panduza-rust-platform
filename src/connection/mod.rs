use futures::FutureExt;
use tokio::sync::mpsc;
use rumqttc::MqttOptions;
use rumqttc::AsyncClient;


use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use std::collections::LinkedList;


use crate::platform::PlatformTaskResult;
use crate::platform::TaskPoolLoader;
use crate::subscription;
use crate::subscription::Filter as SubscriptionFilter;
use crate::subscription::Request as SubscriptionRequest;


mod manager;
mod connection;

pub type Manager = manager::Manager;
pub type AmManager = manager::AmManager;

pub type Connection = connection::Connection;
pub type AmConnection = connection::AmConnection;
