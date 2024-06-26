#[derive(Debug, thiserror::Error, Default, PartialEq, Clone)]
pub enum Error {
    #[error("Mixed use of tabs and spaces for indentation.")]
    MixedIndent,

    #[error("Used space character for indentation instead of tab as used before in the file.")]
    SpaceIndent,

    #[error("Used tab character for indentation instead of space as used before in the file.")]
    TabIndent,

    #[error("Expected another \" at the end of the string literal.")]
    UnclosedDoubleStringLiteral,

    #[error("Expected another \' at the end of the string literal.")]
    UnclosedSingleStringLiteral,

    #[error("Unknown character sequence.")]
    #[default]
    UnknownCharacterSequence,
}
