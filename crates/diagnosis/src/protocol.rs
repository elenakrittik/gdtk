use crate::diagnostic::Diagnostic;

/// A trait for visualizing diagnostics.
pub trait Visualizer<'a> {
    /// The error type, if visualizing a diagnostic can fail.
    type Error: std::error::Error;

    /// Visualize a diagnostic.
    fn visualize(
        &self,
        diag: Diagnostic<'_>,
        f: &mut impl std::io::Write,
    ) -> Result<(), Self::Error>;
}
