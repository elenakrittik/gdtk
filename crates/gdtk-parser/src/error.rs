#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Expected {0}.")]
    Expected(String),

    #[error("Mixed use of spaces and tabs for indentation.")]
    MixedIndent,

    #[error("Unexpected indentation level: expected {0}, found {1}.")]
    UnexpectedIndent(usize, usize),
}
