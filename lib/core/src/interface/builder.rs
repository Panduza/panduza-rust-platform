use crate::Node;

use super::Interface;

pub struct InterfaceBuilder {
    pub parent: Node,
    pub name: String,
}

impl InterfaceBuilder {
    pub fn new<N: Into<String>>(parent: Node, name: N) -> Self {
        Self {
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
