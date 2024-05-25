use crate::diagnostic::Diagnostic;

/// A trait for visualizing diagnostics.
pub trait Visualizer<'a, F: std::io::Write> {
    /// The error type, if visualizing a diagnostic can fail.
    type Error: std::error::Error;

    /// Visualize a diagnostic.
    fn visualize(&self, diag: Diagnostic<'_>, f: &mut F) -> Result<(), Self::Error>;
}
