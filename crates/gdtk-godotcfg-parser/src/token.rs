use crate::error::Error;

pub type Span = std::ops::Range<usize>;

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub span: Span,
}

macro_rules! expectations {
    ($($vis:vis fn $fun:ident -> ($ret:ty) { $arm:pat => $val:expr, _ => $msg:expr, })*) => {
        $($vis fn $fun(self) -> Result<$ret, Error<'a>> {
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
        pub fn expect_identifier -> (&'a str) {
            TokenKind::Identifier(ident) => ident,
            _ => "an identifier",
        }

        pub fn expect_path -> (&'a str) {
            TokenKind::Path(path) => path,
            _ => "a path",
        }

        pub fn expect_path_like -> (&'a str) {
            TokenKind::Identifier(path) | TokenKind::Path(path) => path,
            _ => "an identifier or a path",
        }

        pub fn expect_integer -> (i32) {
            TokenKind::Integer(int) => int,
            _ => "an integer",
        }

        pub fn expect_float -> (f64) {
            TokenKind::Float(float) => float,
            _ => "a float",
        }

        pub fn expect_string -> (&'a str) {
            TokenKind::String(string) => string,
            _ => "a string",
        }

        pub fn expect_boolean -> (bool) {
            TokenKind::Boolean(boolean) => boolean,
            _ => "a boolean",
        }

        pub fn expect_null -> (()) {
            TokenKind::Null => (),
            _ => "null",
        }

        pub fn expect_colon -> (()) {
            TokenKind::Colon => (),
            _ => "a colon",
        }

        pub fn expect_comma -> (()) {
            TokenKind::Comma => (),
            _ => "a comma",
        }

        pub fn expect_assignment -> (()) {
            TokenKind::Assignment => (),
            _ => "an assignment",
        }

        pub fn expect_opening_parenthesis -> (()) {
            TokenKind::OpeningParenthesis => (),
            _ => "a parenthesis",
        }

        pub fn expect_closing_parenthesis -> (()) {
            TokenKind::ClosingParenthesis => (),
            _ => "a parenthesis",
        }

        pub fn expect_opening_bracket -> (()) {
            TokenKind::OpeningBracket => (),
            _ => "a bracket",
        }

        pub fn expect_closing_bracket -> (()) {
            TokenKind::ClosingBracket => (),
            _ => "a bracket",
        }

        pub fn expect_opening_brace -> (()) {
            TokenKind::OpeningBrace => (),
            _ => "a brace",
        }

        pub fn expect_closing_brace -> (()) {
            TokenKind::ClosingBrace => (),
            _ => "a brace",
        }

        pub fn expect_comment -> (&'a str) {
            TokenKind::Comment(comment) => comment,
            _ => "a comment",
        }
    }

    delegate::delegate! {
        to self.kind {
            pub fn is_identifier(&self) -> bool;
            pub fn is_path(&self) -> bool;
            pub fn is_integer(&self) -> bool;
            pub fn is_float(&self) -> bool;
            pub fn is_string(&self) -> bool;
            pub fn is_boolean(&self) -> bool;
            pub fn is_null(&self) -> bool;
            pub fn is_colon(&self) -> bool;
            pub fn is_comma(&self) -> bool;
            pub fn is_assignment(&self) -> bool;
            pub fn is_opening_parenthesis(&self) -> bool;
            pub fn is_closing_parenthesis(&self) -> bool;
            pub fn is_opening_bracket(&self) -> bool;
            pub fn is_closing_bracket(&self) -> bool;
            pub fn is_opening_brace(&self) -> bool;
            pub fn is_closing_brace(&self) -> bool;
            pub fn is_comment(&self) -> bool;
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, logos::Logos, enum_as_inner::EnumAsInner)]
#[logos(error = crate::error::Error<'s>)]
#[logos(skip r"[ \t\n\r\f]+")]
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

    #[regex(";[^\r\n]*\n?", |lex| &lex.slice()[1..])]
    Comment(&'a str),
}
