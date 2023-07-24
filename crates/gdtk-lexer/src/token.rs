use logos::Logos;

use crate::{
    callbacks::{parse_float, parse_integer, trim_comment, trim_string},
    error::SpannedError,
    state::State,
};

// Some reference materials:
// - https://github.com/godotengine/godot/blob/master/modules/gdscript/gdscript_tokenizer.cpp
// - everything mentioned at https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/index.html
// Note that we do not and will not (unless deemed necessary) 1/1 match Godot's token set and/or naming.

// i literally feel how my "chars per LOC" stat goes lower and lower with each token definiton..
#[rustfmt::skip]
#[derive(Logos, Debug, PartialEq)]
#[logos(error = SpannedError, extras = State)]
#[logos(subpattern int = "[0-9]+")]
#[logos(subpattern nint = "[-]*(?&int)")]
pub enum Token<'a> {
    /* Essentials */
    
    #[regex("[_a-zA-Z][_a-zA-Z0-9]*", |lex| lex.slice())] // TODO: UAX-31 identifiers
    Identifier(&'a str),

    /* Literals */
    
    #[regex("(?&nint)", parse_integer)]
    Integer(i64),

    #[regex("(?&nint)(?&int)", parse_float)]
    Float(f64),

    #[regex("\"[^\"]*\"", trim_string)]
    String(&'a str),

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

    #[token("void")]
    Void,

    /* Punctuation */
    
    #[regex("@")]
    Annotation,

    #[token("(")]
    OpenParenthesis,

    #[token(")")]
    ClosingParenthesis,

    #[token("[")]
    OpenBracket,

    #[token("]")]
    ClosingBracket,

    #[token("{")]
    OpenBrace,

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

    #[token("_")]
    Wildcard,

    /* Whitespace */

    #[regex("(\r\n)|(\n)")]
    Newline,

    // these three emitted manually from Blank
    Indent,
    Dedent,
    Spaces,

    #[regex("([ ]|[\t])+")]
    Blank(&'a str), // handled by gdtk-indent

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
}
