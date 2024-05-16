
/// Logger for connections
/// 
#[derive(Clone)]
pub struct Logger {
    /// Connection name
    name: String,
}

///
impl Logger {

    /// Create a new logger
    /// 
    pub fn new<A: Into<String>>(name: A) -> Logger {
        return Logger {
            name: name.into()
        }
    }

    // -- LOGS --

    /// Log trace
    ///
    #[inline]
    pub fn log_warn<A: Into<String>>(&self, text: A) {
        tracing::warn!(class="Connection", cname=self.name, "{}", text.into());
    }

    // /// Log info
    // ///
    // #[inline]
    // pub fn log_info<A: Into<String>>(&self, text: A) {
    //     tracing::info!(class="Interface", bname=self.bench_name, dname=self.dev_name, iname=self.name, 
    //         "{}", text.into());
    // }

    // /// Log debug
    // ///
    // #[inline]
    // pub fn log_debug<A: Into<String>>(&self, text: A) {
    //     tracing::debug!(class="Interface", bname=self.bench_name, dname=self.dev_name, iname=self.name, 
    //         "{}", text.into());
    // }

    /// Log trace
    ///
    #[inline]
    pub fn log_trace<A: Into<String>>(&self, text: A) {
        tracing::trace!(class="Connection", cname=self.name, "{}", text.into());
    }

}

