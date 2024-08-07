/// Generic way to build logs on the platform
///
#[derive(Clone)]
pub struct GenericLogger {
    class: String,
    i1: String,
    i2: String,
    i3: String,
}
impl GenericLogger {
    /// Create a new logger
    ///
    pub fn new<A: Into<String>, B: Into<String>, C: Into<String>, D: Into<String>>(
        class: A,
        i1: B,
        i2: C,
        i3: D,
    ) -> GenericLogger {
        return GenericLogger {
            class: class.into(),
            i1: i1.into(),
            i2: i2.into(),
            i3: i3.into(),
        };
    }

    pub fn error<A: Into<String>>(&self, text: A) {
        tracing::error!(
            class = self.class,
            i1 = self.i1,
            i2 = self.i2,
            i3 = self.i3,
            "{}",
            text.into()
        );
    }

    pub fn warn<A: Into<String>>(&self, text: A) {
        tracing::warn!(
            class = self.class,
            i1 = self.i1,
            i2 = self.i2,
            i3 = self.i3,
            "{}",
            text.into()
        );
    }

    pub fn info<A: Into<String>>(&self, text: A) {
        tracing::info!(
            class = self.class,
            i1 = self.i1,
            i2 = self.i2,
            i3 = self.i3,
            "{}",
            text.into()
        );
    }

    pub fn debug<A: Into<String>>(&self, text: A) {
        tracing::debug!(
            class = self.class,
            i1 = self.i1,
            i2 = self.i2,
            i3 = self.i3,
            "{}",
            text.into()
        );
    }

    pub fn trace<A: Into<String>>(&self, text: A) {
        tracing::trace!(
            class = self.class,
            i1 = self.i1,
            i2 = self.i2,
            i3 = self.i3,
            "{}",
            text.into()
        );
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct PlatformLogger {
    base: GenericLogger,
}
impl PlatformLogger {
    pub fn new() -> PlatformLogger {
        PlatformLogger {
            base: GenericLogger::new("Platform", "", "", ""),
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
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct FactoryLogger {
    base: GenericLogger,
}
impl FactoryLogger {
    pub fn new() -> FactoryLogger {
        FactoryLogger {
            base: GenericLogger::new("Factory", "", "", ""),
        }
    }
    pub fn info<A: Into<String>>(&self, text: A) {
        self.base.info(text);
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct DeviceLogger {
    base: GenericLogger,
}
impl DeviceLogger {
    pub fn new<A: Into<String>>(name: A) -> DeviceLogger {
        DeviceLogger {
            base: GenericLogger::new("Device", name.into(), "", ""),
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
}
