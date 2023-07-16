#[derive(Debug, thiserror::Error)]
pub enum SparsecError {
    #[error("EOF reached while trying to read.")]
    EndOfFile,

    #[error("End-of-input reached while trying to read.")]
    EndOfInput,

    #[error("Start-of-input reached while trying to read.")]
    StartOfInput,

    #[error("Invalid argument(s) provided.")]
    BadArguments { reason: &'static str },

    #[error("Internal parser error. It's not your fault, please report this at https://github.com/elenakrittik/gdtk/issues/")]
    InternalParserError { details: String },

    #[error("All choice subparsers failed.")]
    ChoiceFailed,

    #[error("Unexpected character encountered.")]
    UnexpectedCharacter { expected: char, encountered: char },
}
