use tracing::field::Visit;
use tracing_core::Field;
use std::collections::HashMap;

/// Convert fields of events to a hash map by visiting each fields
/// 
/// ```rust
///    let mut visitor = HashVisitor::new();
///    event.record(&mut visitor);
///    visitor.entries() => map of fields
/// ```
/// 
#[derive(Debug)]
pub struct HashVisitor {
    entries: HashMap<String, String>,
}

impl HashVisitor {
    pub fn new() -> Self {
        HashVisitor {
            entries: HashMap::new(),
        }
    }
    pub fn entries(&self) -> &HashMap<String, String> {
        &self.entries
    }
}

impl Visit for HashVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.entries.insert(field.name().to_string(), format!("{:?}", value));
    }
}
