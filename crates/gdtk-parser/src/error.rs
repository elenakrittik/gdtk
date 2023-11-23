use gdtk_diag::{Diagnostic, Span};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Expected {0}.")]
    Expected(String),

    #[error("Mixed use of spaces and tabs for indentation.")]
    MixedIndent,

    #[error("Unexpected indentation level: expected {0}, found {1}.")]
    UnexpectedIndent(usize, usize),
}

pub trait IntoDiag {
    fn into_diag(self) -> Diagnostic;
}

impl IntoDiag for (Error, Span) {
    fn into_diag(self) -> Diagnostic {
        Diagnostic {
            kind: gdtk_diag::DiagnosticKind::Error,
            message: self.0.to_string(),
            hint: None,
            span: self.1,
        }
    }
}
