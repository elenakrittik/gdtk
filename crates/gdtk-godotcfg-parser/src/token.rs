pub type Span = std::ops::Range<usize>;

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub span: Span,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind<'a>, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn into_identifier(self) -> Result<&'a str, Self> {
        match self.kind {
            TokenKind::Identifier(ident) => Ok(ident),
            _ => Err(self),
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, logos::Logos, enum_as_inner::EnumAsInner)]
#[logos(error = crate::error::Error<'s>)]
#[logos(subpattern segment = "[a-zA-Z_][a-zA-Z0-9_]*")]
pub enum TokenKind<'a> {
    /* Literals */

    #[regex("(?&segment)")]
    Identifier(&'a str),

    #[regex("(?&segment)(/(?&segment))+")]
    Path(&'a str),

    #[regex("-*[0-9]+")]
    Integer,

    #[regex("-*[0-9]+\\.[0-9]+")]
    Float,

    #[regex("\"[^\"]*\"")]
    String,

    #[regex("true|false")]
    Boolean,

    #[token("null")]
    Null,

    /* Symbols */

    #[token("=")]
    Assignment,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("(")]
    OpeningParenthesis,

    #[token(")")]
    ClosingParenthesis,

    #[token("[")]
    OpeningBracket,

    #[token("]")]
    ClosingBracket,

    #[token("{")]
    OpeningBrace,

    #[token("}")]
    ClosingBrace,

    /* Specials */

    #[regex(";[^\n]*")]
    Comment(&'a str),
}
