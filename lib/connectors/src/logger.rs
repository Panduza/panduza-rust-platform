static CLASS_NAME: &str = "Connector";

/// Logger for interfaces
/// 
#[derive(Clone)]
pub struct Logger {
    /// Bench name
    key: String,
}
impl Logger {

    /// Create a new logger
    ///
    pub fn new<A: Into<String>>(key: A) 
        -> Logger {
        return Logger {
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
        tracing::warn!(class=CLASS_NAME, key=self.key, "{}", text.into());
    }

    #[inline]
    pub fn log_info<A: Into<String>>(&self, text: A) {
        tracing::info!(class=CLASS_NAME, key=self.key, "{}", text.into());
    }

    #[inline]
    pub fn log_trace<A: Into<String>>(&self, text: A) {
        tracing::trace!(class=CLASS_NAME, key=self.key, "{}", text.into());
    }

}

