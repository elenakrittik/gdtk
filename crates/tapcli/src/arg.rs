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

impl std::fmt::Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
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

impl std::fmt::Display for ArgRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Short(c) => write!(f, "-{}", c),
            Self::Long(s) => write!(f, "--{}", s),
            Self::Value(s) => write!(f, "{}", s),
        }
    }
}
