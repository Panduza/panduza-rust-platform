static CLASS_NAME: &str = "Connector";

/// Logger for Connector Gates
/// 
#[derive(Clone)]
pub struct Logger {
    /// Connector name
    name: String,
    /// Connector unique key instance
    key: String,
}
impl Logger {

    /// Create a new Logger
    ///
    pub fn new<A: Into<String>, B: Into<String>>(name: A, key: B) 
        -> Logger {
        return Logger {
            name: name.into(),
            key: key.into()
        }
    }

    // -- LOGS --

    // #[inline]
    // pub fn log_error<A: Into<String>>(&self, text: A) {
    //     tracing::error!(class=CLASS_NAME, bname=self.bname, dname=self.dname, iname=self.iname, "{}", text.into());
    // }

    #[inline]
    pub fn log_warn<A: Into<String>>(&self, text: A) {
        tracing::warn!(class=CLASS_NAME, i1=self.name, i2=self.key, "{}", text.into());
    }

    #[inline]
    pub fn log_info<A: Into<String>>(&self, text: A) {
        tracing::info!(class=CLASS_NAME, i1=self.name, i2=self.key, "{}", text.into());
    }

    #[inline]
    pub fn log_trace<A: Into<String>>(&self, text: A) {
        tracing::trace!(class=CLASS_NAME, i1=self.name, i2=self.key, "{}", text.into());
    }

}

