static CLASS_NAME: &str = "Connector";

/// Logger for Connector Gates
/// 
#[derive(Clone)]
pub struct Logger {
    /// Connector name
    name: String,
}
impl Logger {

    /// Create a new Logger
    ///
    pub fn new<A: Into<String>>(name: A) 
        -> Logger {
        return Logger {
            name: name.into()
        }
    }

    // -- LOGS --

    #[inline]
    pub fn log_warn<A: Into<String>>(&self, text: A) {
        tracing::warn!(class=CLASS_NAME, i1=self.name, "{}", text.into());
    }

    #[inline]
    pub fn log_info<A: Into<String>>(&self, text: A) {
        tracing::info!(class=CLASS_NAME, i1=self.name, "{}", text.into());
    }

    #[inline]
    pub fn log_debug<A: Into<String>>(&self, text: A) {
        tracing::debug!(class=CLASS_NAME, i1=self.name, "{}", text.into());
    }

    #[inline]
    pub fn log_trace<A: Into<String>>(&self, text: A) {
        tracing::trace!(class=CLASS_NAME, i1=self.name, "{}", text.into());
    }

}



