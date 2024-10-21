use std::collections::HashMap;

use panduza_platform_core::device::State;

use super::InfoElement;

///
///
///
pub struct InfoElementInstance {
    ///
    ///
    ///
    pub name: String,

    pub state: State,

    ///
    ///
    ///
    pub children: HashMap<String, InfoElement>,
}

impl InfoElementInstance {
    pub fn new<A: Into<String>>(name: A) -> Self {
        Self {
            name: name.into(),
            state: State::Booting,
            children: HashMap::new(),
        }
    }

    pub fn set_state(&mut self, new_state: State) {
        self.state = new_state;
    }
}
