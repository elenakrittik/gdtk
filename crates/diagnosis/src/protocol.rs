use crate::diagnostic::Diagnostic;


/// A trait for visualizing diagnostics.
pub trait Visualizer {
    /// The error type, if visualizing a diagnostic can fail.
    type Error: std::error::Error;

    /// Visualize a diagnostic.
    fn visualize(&self, report: Diagnostic<'_>, buf: &mut impl std::fmt::Write) -> Result<(), Self::Error>;
}

/// A trait for visualizing diagnostics with access to the source (code).
pub trait SourceVisualizer<'a>: Visualizer {
    /// Get the source (code).
    fn source(&'a self) -> Result<&'a str, Self::Error>;
}
