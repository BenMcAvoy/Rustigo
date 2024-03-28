/// A pattern is a string that can be used to match keys.
/// It is used to match request paths to routes.
#[derive(PartialEq, Eq, Hash)]
pub struct Pattern {
    pub(crate) pattern: String,
}

impl Pattern {
    /// Create a new pattern with the given string.
    pub fn new(pattern: &str) -> Pattern {
        Pattern {
            pattern: pattern.to_string(),
        }
    }

    /// Check if the pattern matches the given path.
    pub fn matches(&self, key: &str) -> bool {
        let pattern = &self.pattern;

        if pattern.contains('*') {
            let pattern = pattern.strip_suffix('*').unwrap();

            key.starts_with(pattern)
        } else {
            pattern == key
        }
    }
}
