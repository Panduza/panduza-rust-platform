static CLASS_NAME: &str = "Interface";

/// Logger for interfaces
/// 
#[derive(Clone)]
pub struct Logger {
    /// Bench name
    bname: String,
    /// Device name
    dname: String,
    /// Interface name
    iname: String,
}
impl Logger {

    /// Create a new logger
    ///
    pub fn new<A: Into<String>, B: Into<String>, C: Into<String>>(bname: A, dname: B, iname: C) -> Logger {
        return Logger {
            bname: bname.into(),
            dname: dname.into(),
            iname: iname.into()
        }
    }

    // -- LOGS --

    // #[inline]
    // pub fn log_error<A: Into<String>>(&self, text: A) {
    //     tracing::error!(class=CLASS_NAME, bname=self.bname, dname=self.dname, iname=self.iname, "{}", text.into());
    // }

    #[inline]
    pub fn log_warn<A: Into<String>>(&self, text: A) {
        tracing::warn!(class=CLASS_NAME, bname=self.bname, dname=self.dname, iname=self.iname, "{}", text.into());
    }

    #[inline]
    pub fn log_info<A: Into<String>>(&self, text: A) {
        tracing::info!(class=CLASS_NAME, bname=self.bname, dname=self.dname, iname=self.iname, "{}", text.into());
    }

    #[inline]
    pub fn log_trace<A: Into<String>>(&self, text: A) {
        tracing::trace!(class=CLASS_NAME, bname=self.bname, dname=self.dname, iname=self.iname, "{}", text.into());
    }

}

