use crate::Error;

pub enum AttributeMode {
    AttOnly,
    CmdOnly,
    Bidir,
}

pub struct ElementAttribute {
    name: String,
    typee: String,
    mode: AttributeMode,
}

impl ElementAttribute {
    ///
    ///
    ///
    pub fn new<N: Into<String>, T: Into<String>>(name: N, typee: T, mode: AttributeMode) -> Self {
        Self {
            name: name.into(),
            typee: typee.into(),
            mode,
        }
    }
    ///
    pub fn name(&self) -> &String {
        &self.name
    }
    ///
    pub fn typee(&self) -> &String {
        &self.typee
    }
    pub fn mode(&self) -> &AttributeMode {
        &self.mode
    }

    ///
    /// Attribute does not hold any elements
    ///
    pub fn is_element_exist(&self, layers: Vec<String>) -> Result<bool, Error> {
        Ok(false)
    }
}
