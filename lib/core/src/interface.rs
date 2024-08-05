pub mod builder;

pub struct Interface {
    name: String,
}

impl Interface {
    pub fn create_attribute<N: Into<String>>(&mut self, name: N) {}
}

impl From<builder::InterfaceBuilder> for Interface {
    fn from(builder: builder::InterfaceBuilder) -> Self {
        Interface { name: builder.name }
    }
}
