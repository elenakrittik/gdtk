use std::fmt::Display;

#[derive(Debug, thiserror::Error, Default, PartialEq, Clone)]
pub enum Error {
    #[error("Mixed use of tabs and spaces for indentation.")]
    MixedIndent,

    #[error("Used space character for indentation instead of tab as used before in the file.")]
    SpaceIndent,

    #[error("Used tab character for indentation instead of space as used before in the file.")]
    TabIndent,

    #[error("Unknown character.")]
    #[default]
    UnknownCharacter,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SpannedError(pub Error, pub std::ops::Range<usize>);

impl Display for SpannedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[macro_export]
macro_rules! spanned {
    ($lex: expr, $err: expr) => {
        $crate::error::SpannedError($err, $lex.span())
    };
}
