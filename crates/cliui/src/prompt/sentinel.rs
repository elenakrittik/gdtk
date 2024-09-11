/// A special type that all prompt-related `new()` functions use in place of generic parameters.
pub struct Sentinel(());

impl std::fmt::Display for Sentinel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DisplaySentinel")
    }
}

impl Iterator for Sentinel {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
