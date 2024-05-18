use crate::error::Error;

pub type Span = std::ops::Range<usize>;

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub span: Span,
}

macro_rules! expectations {
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

    expectations! {
        expect_identifier -> (&'a str):
            TokenKind::Identifier(ident) => ident,
            _ => "an identifier",

        expect_path -> (&'a str):
            TokenKind::Path(path) => path,
            _ => "a path",

        expect_path_like -> (&'a str):
            TokenKind::Identifier(path) | TokenKind::Path(path) => path,
            _ => "an identifier or a path",

        expect_integer -> (i32):
            TokenKind::Integer(int) => int,
            _ => "an integer",

        expect_float -> (f64):
            TokenKind::Float(float) => float,
            _ => "a float",

        expect_string -> (&'a str):
            TokenKind::String(string) => string,
            _ => "a string",

        expect_boolean -> (bool):
            TokenKind::Boolean(boolean) => boolean,
            _ => "a boolean",

        expect_null -> (()):
            TokenKind::Null => (),
            _ => "null",

        expect_colon -> (()):
            TokenKind::Colon => (),
            _ => "a colon",

        expect_comma -> (()):
            TokenKind::Comma => (),
            _ => "a comma",

        expect_assignment -> (()):
            TokenKind::Assignment => (),
            _ => "an assignment",

        expect_opening_parenthesis -> (()):
            TokenKind::OpeningParenthesis => (),
            _ => "a parenthesis",

        expect_closing_parenthesis -> (()):
            TokenKind::ClosingParenthesis => (),
            _ => "a parenthesis",

        expect_opening_bracket -> (()):
            TokenKind::OpeningBracket => (),
            _ => "a bracket",

        expect_closing_bracket -> (()):
            TokenKind::ClosingBracket => (),
            _ => "a bracket",

        expect_opening_brace -> (()):
            TokenKind::OpeningBrace => (),
            _ => "a brace",

        expect_closing_brace -> (()):
            TokenKind::ClosingBrace => (),
            _ => "a brace",
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, logos::Logos, enum_as_inner::EnumAsInner)]
#[logos(error = crate::error::Error<'s>)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(subpattern segment = "[a-zA-Z_][a-zA-Z0-9_]*")]
pub enum TokenKind<'a> {
    /* Literals */

    #[regex("(?&segment)")]
    Identifier(&'a str),

    #[regex("(?&segment)(/(?&segment))+")]
    Path(&'a str),

    #[regex("-*[0-9]+", |lex| lex.slice().parse::<i32>().ok())]
    Integer(i32),

    #[regex("-*[0-9]+\\.[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    Float(f64),

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
