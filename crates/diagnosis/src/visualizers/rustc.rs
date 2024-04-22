use std::convert::Infallible;

use crate::{diagnostic::Diagnostic, protocol::{SourceVisualizer, Visualizer}};

/// A visualizer that visualizes diagnostics in rustc's fashion.
pub struct RustcVisualizer<'a> {
    source: &'a str,
}

impl<'a> RustcVisualizer<'a> {
    /// Create a new visualizer.
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }
}

impl<'a> Visualizer for RustcVisualizer<'a> {
    type Error = Infallible;
    
    fn visualize(&self, _report: Diagnostic<'_>, _buf: &mut impl std::fmt::Write) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<'a> SourceVisualizer<'a> for RustcVisualizer<'a> {
    fn source(&'a self) -> Result<&'a str, Self::Error> {
        Ok(self.source)
    }
}
