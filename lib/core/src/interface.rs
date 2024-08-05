use crate::{AttributeBuilder, Reactor};

pub mod builder;

pub struct Interface {
    ///
    reactor: Reactor,
    ///
    topic: String,
}

impl Interface {
    pub fn create_attribute<N: Into<String>>(&mut self, name: N) -> AttributeBuilder {
        self.reactor
            .create_new_attribute()
            .with_topic(format!("{}/{}", self.topic, name.into()))
    }
}

impl From<builder::InterfaceBuilder> for Interface {
    fn from(builder: builder::InterfaceBuilder) -> Self {
        Interface {
            reactor: builder.reactor,
            topic: builder.topic,
        }
    }
}
