/// An argument.
#[derive(Debug)]
pub enum Arg {
    /// A short argument.
    Short(char),
    /// A long argument.
    Long(String),
    /// A value.
    Value(String),
}

impl Arg {
    /// Get an [ArgRef] for this argument.
    pub fn as_ref(&self) -> ArgRef<'_> {
        match self {
            Arg::Short(c) => ArgRef::Short(c),
            Arg::Long(s) => ArgRef::Long(s),
            Arg::Value(s) => ArgRef::Value(s),
        }
    }
}

/// A borrowed version of [Arg] useful primarily for pattern
/// matching.
#[derive(Debug)]
pub enum ArgRef<'a> {
    /// See [Arg::Short].
    Short(&'a char),
    /// See [Arg::Long].
    Long(&'a str),
    /// See [Arg::Value].
    Value(&'a str),
}
