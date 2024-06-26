use logos::Logos;

use crate::lexer::callbacks::{convert, convert_radix, trim_quotes};
use crate::lexer::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub span: gdtk_span::Span,
    pub kind: TokenKind<'a>,
}

impl<'a> Token<'a> {
    pub fn transmute(self, new_kind: TokenKind<'a>) -> Token<'a> {
        Self {
            span: self.span,
            kind: new_kind,
        }
    }
}

// Some reference materials:
// - https://github.com/godotengine/godot/blob/master/modules/gdscript/gdscript_tokenizer.cpp
// - everything mentioned at https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/index.html
// Note that we do not and will not (unless deemed necessary) 1/1 match Godot's token set and/or naming.

#[rustfmt::skip]
#[derive(Logos, Debug, PartialEq, Clone, enum_as_inner::EnumAsInner)]
#[logos(error = Error)]
#[logos(subpattern int = r"[0-9](_?[0-9])*_?")]
#[logos(subpattern float = r"(?&int)\.(?&int)")]
#[logos(subpattern string = "(\"[^\"\r\n]*\")|('[^'\r\n]*')")]
#[logos(subpattern newline = "(\r\n)|(\n)")]
pub enum TokenKind<'a> {
    /* Literals */

    // TODO: multiline strings

    #[regex(r"(\p{XID_Start}|_)\p{XID_Continue}*")]
    Identifier(&'a str),

    #[regex("(?&int)", convert)]
    Integer(u64),

    #[regex("0b[01](_?[01])*", convert_radix::<2>)]
    BinaryInteger(u64),

    #[regex("0x[0-9abcdefABCDEF](_?[0-9abcdefABCDEF])*", convert_radix::<16>)]
    HexInteger(u64),

    #[regex(r"(?&float)[eE][+-]?(?&int)", convert)]
    ScientificFloat(f64),

    #[regex(r"(?&float)", convert)]
    Float(f64),

    #[regex("(?&string)", trim_quotes::<false>)]
    String(&'a str),

    #[regex("\\&(?&string)", trim_quotes::<true>)]
    StringName(&'a str),

    #[regex("\\$(?&string)", trim_quotes::<true>)]
    Node(&'a str),

    #[regex("%(?&string)", trim_quotes::<true>)]
    UniqueNode(&'a str),

    #[regex("\\^(?&string)", trim_quotes::<true>)]
    NodePath(&'a str),

    #[regex("true|false", convert)]
    Boolean(bool),

    #[token("null")]
    Null,

    /* Comparison */

    #[token("<")]
    LessThan,

    #[token("<=")]
    LessThanOrEqual,

    #[token(">")]
    GreaterThan,

    #[token(">=")]
    GreaterThanOrEqual,

    #[token("==")]
    Equal,

    #[token("!=")]
    NotEqual,

    /* Logical */

    #[token("and")]
    And,

    #[token("or")]
    Or,

    #[token("not")]
    Not,

    #[token("&&")]
    SymbolizedAnd,

    #[token("||")]
    SymbolizedOr,

    #[token("!")]
    SymbolizedNot,

    /* Bitwise operators */

    #[token("&")]
    BitwiseAnd,

    #[token("|")]
    BitwiseOr,

    #[token("~")]
    BitwiseNot,

    #[token("^")]
    BitwiseXor,

    #[token("<<")]
    BitwiseShiftLeft,

    #[token(">>")]
    BitwiseShiftRight,

    /* Math */

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("**")]
    Power,

    #[token("/")]
    Divide,

    #[token("%")]
    Remainder,

    /* Other operators */

    #[token("as")]
    As,

    #[token("await")]
    Await,

    #[token("in")]
    In,

    // NOTE: `not in` is generated manually in `crate::lex`
    NotIn,

    #[token("is")]
    Is,

    /* Assignment */

    #[token("=")]
    Assignment,

    #[token("+=")]
    PlusAssignment,

    #[token("-=")]
    MinusAssignment,

    #[token("*=")]
    MultiplyAssignment,

    #[token("**=")]
    PowerAssignment,

    #[token("/=")]
    DivideAssignment,

    #[token("%=")]
    RemainderAssignment,

    #[token("&=")]
    BitwiseAndAssignment,

    #[token("|=")]
    BitwiseOrAssignment,

    #[token("~=")]
    BitwiseNotAssignment,

    #[token("^=")]
    BitwiseXorAssignment,

    #[token("<<=")]
    BitwiseShiftLeftAssignment,

    #[token(">>=")]
    BitwiseShiftRightAssignment,

    /* Control flow */

    #[token("if")]
    If,

    #[token("elif")]
    Elif,

    #[token("else")]
    Else,

    #[token("for")]
    For,

    #[token("while")]
    While,

    #[token("break")]
    Break,

    #[token("continue")]
    Continue,

    #[token("pass")]
    Pass,

    #[token("return")]
    Return,

    #[token("match")]
    Match,

    /* Keywords */

    #[token("assert")]
    Assert,

    #[token("breakpoint")]
    Breakpoint,

    #[token("class")]
    Class,

    #[token("class_name")]
    ClassName,

    #[token("const")]
    Const,

    #[token("enum")]
    Enum,

    #[token("extends")]
    Extends,

    #[token("func")]
    Func,

    #[token("signal")]
    Signal,

    #[token("static")]
    Static,

    #[token("var")]
    Var,

    #[token("when")]
    When,

    /* Punctuation */

    #[regex("@")]
    Annotation,

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

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token(".")]
    Period,

    #[token("..")]
    Range,

    #[token(":")]
    Colon,

    #[token("$")]
    Dollar,

    #[token("->")]
    Arrow,

    /* Whitespace */

    #[regex("\\\\(?&newline)", logos::skip)]
    NewlineEscape,

    #[regex("(?&newline)")]
    Newline,

    #[regex("([ ]|[\t])+")]
    Blank(&'a str),

    // these two are generated manually
    Indent,
    Dedent,

    /* Specials */

    #[regex("#[^\n]*")]
    Comment(&'a str),

    /* Reserved and deprecated tokens */

    #[token("namespace")]
    Namespace,

    #[token("trait")]
    Trait,

    #[token("yield")]
    Yield,

    /* We don't do that here */

    // #[token("preload")]
    // Preload,

    // #[token("self")]
    // Self,

    // #[token("_")]
    // Wildcard,

    // #[token("void")]
    // Void,
}

// In my humble opinion, a matches! here is less readable.
#[allow(clippy::match_like_matches_macro)]
impl TokenKind<'_> {
    pub fn is_any_assignment(&self) -> bool {
        match self {
            TokenKind::PlusAssignment
            | TokenKind::MinusAssignment
            | TokenKind::MultiplyAssignment
            | TokenKind::PowerAssignment
            | TokenKind::DivideAssignment
            | TokenKind::RemainderAssignment
            | TokenKind::BitwiseAndAssignment
            | TokenKind::BitwiseOrAssignment
            | TokenKind::BitwiseNotAssignment
            | TokenKind::BitwiseXorAssignment
            | TokenKind::BitwiseShiftLeftAssignment
            | TokenKind::BitwiseShiftRightAssignment => true,
            _ => false,
        }
    }

    pub fn is_line_end(&self) -> bool {
        match self {
            TokenKind::Newline
            | TokenKind::ClosingBrace
            | TokenKind::ClosingBracket
            | TokenKind::ClosingParenthesis
            | TokenKind::Semicolon
            | TokenKind::Dedent => true,
            _ => false,
        }
    }

    pub fn same_as(&self, other: &TokenKind<'_>) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

#[derive(Logos, Debug)]
pub(crate) enum CommentLexer<'a> {
    #[regex("(\\\\)?((\r\n)|(\n))")]
    Newline,

    #[regex("#[^\n]*")]
    Comment(&'a str),
}
