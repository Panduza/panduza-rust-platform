
pub struct TimeLock {
    pub duration: tokio::time::Duration,
    pub t0: tokio::time::Instant
}
