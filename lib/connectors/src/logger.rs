use panduza_platform_core::GenericLogger;

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct ConnectorLogger {
    base: GenericLogger,
}
impl ConnectorLogger {
    pub fn new<B: Into<String>, C: Into<String>, D: Into<String>>(
        i1: B,
        i2: C,
        i3: D,
    ) -> ConnectorLogger {
        ConnectorLogger {
            base: GenericLogger::new("Connector", i1, i2, i3),
        }
    }
    pub fn error<A: Into<String>>(&self, text: A) {
        self.base.error(text);
    }
    pub fn warn<A: Into<String>>(&self, text: A) {
        self.base.warn(text);
    }
    pub fn info<A: Into<String>>(&self, text: A) {
        self.base.info(text);
    }
    pub fn debug<A: Into<String>>(&self, text: A) {
        self.base.debug(text);
    }
    pub fn trace<A: Into<String>>(&self, text: A) {
        self.base.trace(text);
    }
}
