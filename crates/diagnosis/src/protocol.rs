use crate::diagnostic::Diagnostic;


/// A trait for visualizing diagnostics.
pub trait Visualizer<'a> {
    /// The error type, if visualizing a diagnostic can fail.
    type Error: std::error::Error;

    /// Visualize a diagnostic.
    fn visualize(&self, report: Diagnostic<'_>, buf: &mut impl std::fmt::Write) -> Result<(), Self::Error>;

    /// Get the source (code).
    fn source(&'a self) -> Result<Option<&'a str>, Self::Error>;
}
