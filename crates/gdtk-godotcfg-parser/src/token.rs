use crate::error::Error;

pub type Span = std::ops::Range<usize>;

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub span: Span,
}

macro_rules! expectation {
    ($($fun:ident -> ($ret:ty): $arm:pat => $val:expr, _ => $msg:expr,)*) => {
        $(pub fn $fun(self) -> Result<$ret, Error<'a>> {
            match self.kind {
                $arm => Ok($val),
                _ => Err(Error::Unexpected(self, $msg)),
            }
        })*
    };
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind<'a>, span: Span) -> Self {
        Self { kind, span }
    }

    expectation! {
        expect_identifier -> (&'a str):
            TokenKind::Identifier(ident) => ident,
            _ => "an identifier",

        expect_colon -> (()):
            TokenKind::Colon => (),
            _ => "a colon",
    }

    pub fn expect_identifier_like(self) -> Result<&'a str, Error<'a>> {
        match self.kind {
            TokenKind::Identifier(ident) | TokenKind::Path(ident) => Ok(ident),
            _ => Err(Error::Unexpected(self, "an identifier or a path")),
        }
    }

    pub fn expect_assignment(self) -> Result<(), Error<'a>> {
        match self.kind {
            TokenKind::Assignment => Ok(()),
            _ => Err(Error::Unexpected(self, "an opening bracket")),
        }
    }

    pub fn expect_opening_bracket(self) -> Result<(), Error<'a>> {
        match self.kind {
            TokenKind::OpeningBracket => Ok(()),
            _ => Err(Error::Unexpected(self, "an opening bracket")),
        }
    }

    pub fn expect_closing_bracket(self) -> Result<(), Error<'a>> {
        match self.kind {
            TokenKind::ClosingBracket => Ok(()),
            _ => Err(Error::Unexpected(self, "a closing bracket")),
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

    #[regex("-*[0-9]+", |lex| lex.slice().parse::<i32>().ok())]
    Integer(i32),

    #[regex("-*[0-9]+\\.[0-9]+", |lex| lex.slice().parse::<f32>().ok())]
    Float(f32),

    #[regex("\"[^\"]*\"", |lex| lex.slice().trim_matches('"'))]
    String(&'a str),

    #[regex("true|false", |lex| lex.slice().parse::<bool>().ok())]
    Boolean(bool),

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

    #[regex(";[^\n]*", |lex| &lex.slice()[1..])]
    Comment(&'a str),
}
