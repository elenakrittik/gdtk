#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub span: gdtk_span::Span,
    pub kind: TokenKind<'a>,
}

// Some reference materials:
// - https://github.com/godotengine/godot/blob/master/modules/gdscript/gdscript_tokenizer.cpp
// - everything mentioned at https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/index.html
// Note that we do not and will not (unless deemed necessary) 1/1 match Godot's token set and/or naming.

#[rustfmt::skip]
#[derive(Debug, PartialEq, enum_as_inner::EnumAsInner)]
pub enum TokenKind<'a> {
    /* Literals */

    Identifier(&'a str),
    Integer(u64),
    BinaryInteger(u64),
    HexInteger(u64),
    ScientificFloat(f64),
    Float(f64),
    String(&'a str),
    StringName(&'a str),
    Node(&'a str),
    UniqueNode(&'a str),
    NodePath(&'a str),
    Boolean(bool),
    Null,

    /* Comparison */
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,

    /* Logical operators */
    And,
    Or,
    Not,
    SymbolizedAnd,
    SymbolizedOr,
    SymbolizedNot,

    /* Bitwise operators */
    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    BitwiseXor,
    BitwiseShiftLeft,
    BitwiseShiftRight,

    /* Math */
    Plus,
    Minus,
    Multiply,
    Power,
    Divide,
    Remainder,

    /* Other operators */
    As,
    Await,
    In,
    NotIn,
    Is,

    /* Assignment */
    Assignment,
    PlusAssignment,
    MinusAssignment,
    MultiplyAssignment,
    PowerAssignment,
    DivideAssignment,
    RemainderAssignment,
    BitwiseAndAssignment,
    BitwiseOrAssignment,
    BitwiseNotAssignment,
    BitwiseXorAssignment,
    BitwiseShiftLeftAssignment,
    BitwiseShiftRightAssignment,

    /* Control flow */
    If,
    Elif,
    Else,
    For,
    While,
    Break,
    Continue,
    Pass,
    Return,
    Match,

    /* Keywords */
    Assert,
    Breakpoint,
    Class,
    ClassName,
    Const,
    Enum,
    Extends,
    Func,
    Signal,
    Static,
    Var,
    When,

    /* Punctuation */
    Annotation,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBracket,
    ClosingBracket,
    OpeningBrace,
    ClosingBrace,
    Comma,
    Semicolon,
    Period,
    Range,
    Colon,
    Dollar,
    Arrow,

    /* Whitespace */
    NewlineEscape,
    Newline,
    Blank(&'a str),

    // these two are generated manually
    Indent,
    Dedent,

    /* Specials */
    Comment(&'a str),

    /* Reserved and deprecated tokens */
    Namespace,
    Trait,
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
