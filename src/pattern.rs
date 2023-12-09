#[derive(PartialEq, Eq, Hash)]
pub struct Pattern {
    pub(crate) pattern: String,
}

impl Pattern {
    pub fn new(pattern: &str) -> Pattern {
        Pattern {
            pattern: pattern.to_string(),
        }
    }

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
