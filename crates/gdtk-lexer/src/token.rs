use logos::Logos;

use crate::{
    callbacks::{
        check_indent_style, parse_binary, parse_bool, parse_e_notation, parse_float, parse_hex,
        parse_integer, strip_prefix_and_quotes, strip_quotes, trim_comment,
    },
    error::Error,
};

// Some reference materials:
// - https://github.com/godotengine/godot/blob/master/modules/gdscript/gdscript_tokenizer.cpp
// - everything mentioned at https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/index.html
// Note that we do not and will not (unless deemed necessary) 1/1 match Godot's token set and/or naming.

#[rustfmt::skip]
#[derive(Logos, Debug, PartialEq)]
#[logos(error = Error)]
#[logos(subpattern int = r"[0-9](_?[0-9])*_?")]
#[logos(subpattern float = r"(?&int)\.(?&int)")]
#[logos(subpattern string = "(\"[^\"\r\n]*\")|('[^'\r\n]*')")]
pub enum Token<'a> {
    /* Essentials */
    
    #[regex(r"(\p{XID_Start}|_)\p{XID_Continue}*")]
    Identifier(&'a str),

    /* Literals */

    // TODO: multiline strings

    #[regex("(?&int)", parse_integer)]
    Integer(i64),

    #[regex("0b[01](_?[01])*", parse_binary)]
    BinaryInteger(u64),

    #[regex("0x[0-9abcdefABCDEF](_?[0-9abcdefABCDEF])*", parse_hex)]
    HexInteger(u64),

    #[regex(r"(?&float)[eE][+-](?&int)", parse_e_notation)]
    ScientificFloat(f64),

    #[regex(r"(?&float)", parse_float)]
    Float(f64),

    #[regex("(?&string)", strip_quotes)]
    String(&'a str),

    #[regex("\\&(?&string)", |lex| strip_prefix_and_quotes(lex, '&'))]
    StringName(&'a str),

    #[regex("\\$(?&string)", |lex| strip_prefix_and_quotes(lex, '$'))]
    Node(&'a str),

    #[regex("%(?&string)", |lex| strip_prefix_and_quotes(lex, '%'))]
    UniqueNode(&'a str),

    #[regex("\\^(?&string)", |lex| strip_prefix_and_quotes(lex, '^'))]
    NodePath(&'a str),

    #[regex("true|false", parse_bool)]
    Boolean(bool),

    #[token("null")]
    Null,

    /* Comparison */

    #[token("<")]
    Less,

    #[token("<=")]
    LessEqual,

    #[token(">")]
    Greater,

    #[token(">=")]
    GreaterEqual,

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

    #[token("as")]
    As,

    #[token("assert")]
    Assert,

    #[token("await")]
    Await,

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

    #[token("in")]
    In,

    #[token("is")]
    Is,

    #[token("signal")]
    Signal,

    #[token("static")]
    Static,

    #[token("var")]
    Var,

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

    #[regex("(\r\n)|(\n)")]
    Newline,

    // these three are emitted manually from Blank
    Indent,
    Dedent,
    Spaces,

    #[regex("([ ]|[\t])+", |lex| { unsafe { check_indent_style(lex) } })]
    Blank(&'a str),

    /* Specials */

    #[regex("#[^\n]*", trim_comment)]
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
