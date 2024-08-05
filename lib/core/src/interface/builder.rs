use std::sync::Weak;

use tokio::sync::Mutex;

use crate::{Node, Reactor};

use super::Interface;

pub struct InterfaceBuilder {
    //
    pub reactor: Reactor,
    ///
    pub parent: Weak<Mutex<Node>>,
    ///
    pub name: String,
}

impl InterfaceBuilder {
    pub fn new<N: Into<String>>(reactor: Reactor, parent: Weak<Mutex<Node>>, name: N) -> Self {
        Self {
            reactor: reactor,
            parent: parent,
            name: name.into(),
        }
    }

    pub fn with_tags<T: Into<String>>(self, tags: T) -> Self {
        self
    }

    pub fn finish(self) -> Interface {
        Interface::from(self)
    }
}
