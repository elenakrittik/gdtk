use std::convert::Infallible;

use crate::{
    diagnostic::{Diagnostic, Severity},
    protocol::Visualizer,
};

// TODO: Use fnv here for const hashmaps
const ERROR: yansi::Style = yansi::Style::new().red();
const WARNING: yansi::Style = yansi::Style::new().yellow();
const ADVICE: yansi::Style = yansi::Style::new().cyan();

const fn message_style(severity: Severity) -> yansi::Style {
    match severity {
        Severity::Error => ERROR,
        Severity::Warning => WARNING,
        Severity::Advice => ADVICE,
    }
}

/// A visualizer that visualizes diagnostics in rustc's fashion.
///
/// An example output may look like this:
///
/// ```
/// error[E0499]: cannot borrow `v` as mutable more than once at a time
///  --> src/main.rs:4:15
///   |
/// 3 |     let one = &mut v;
///   |               ------ first mutable borrow occurs here
/// 4 |     let two = &mut v;
///   |               ^^^^^^ second mutable borrow occurs here
/// 5 |
/// 6 |     dbg!(one, two);
///   |          --- first borrow later used here
/// ```
pub struct RustcVisualizer<'a> {
    source: &'a str,
}

impl<'a> RustcVisualizer<'a> {
    /// Create a new visualizer.
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }
}

impl<'a> Visualizer<'a> for RustcVisualizer<'a> {
    type Error = Infallible;

    fn visualize(
        &self,
        _report: Diagnostic<'_>,
        _buf: &mut impl std::fmt::Write,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn source(&'a self) -> Result<Option<&'a str>, Self::Error> {
        Ok(Some(self.source))
    }
}
