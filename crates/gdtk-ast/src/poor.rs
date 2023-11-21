//! "Poor" GDScript abstract syntax tree. Does not interlink any references
//! and instead represents them as raw strings.

pub type CodeBlock<'a> = Vec<ASTStatement<'a>>;

#[derive(Debug)]
pub struct ASTClass<'a> {
    pub class_name: Option<&'a str>,
    pub extends: Option<&'a str>,
    pub icon_path: Option<&'a str>,
    pub variables: Vec<ASTVariable<'a>>,
    pub enums: Vec<ASTEnum<'a>>,
    pub functions: Vec<ASTFunction<'a>>,
}

#[derive(Debug)]
pub struct ASTVariable<'a> {
    pub identifier: &'a str,
    pub typehint: Option<&'a str>,
    pub value: Option<ASTValue<'a>>,
    pub kind: ASTVariableKind,
}

#[derive(Debug)]
pub enum ASTVariableKind {
    /// Regular (`var`) variable.
    Regular,

    /// Constant (`const`) variable.
    Constant,

    /// Static (`static var`) variable.
    Static,
}

#[derive(Debug)]
pub struct ASTEnum<'a> {
    pub identifier: Option<&'a str>,
    pub variants: Vec<ASTEnumVariant<'a>>,
}

#[derive(Debug)]
pub struct ASTEnumVariant<'a> {
    pub identifier: &'a str,
    pub value: Option<i64>,
}

#[derive(Debug)]
pub struct ASTFunction<'a> {
    pub identifier: Option<&'a str>,
    pub parameters: Vec<ASTFunctionParameter<'a>>,
    pub typehint: &'a str,
    pub body: CodeBlock<'a>,
}

#[derive(Debug)]
pub struct ASTFunctionParameter<'a> {
    pub identifier: &'a str,
    pub typehint: Option<&'a str>,
    pub default: Option<ASTValue<'a>>,
}

#[derive(Debug)]
pub enum ASTValue<'a> {
    Identifier(&'a str),
    Number(i64),
    Float(f64),
    String(&'a str),
    StringName(&'a str),
    Node(&'a str),
    UniqueNode(&'a str),
    NodePath(&'a str),
    Group(Vec<ASTValue<'a>>),
    Boolean(bool),
    Array(Vec<ASTValue<'a>>),
    Dictionary(Vec<(&'a str, ASTValue<'a>)>),
    Lambda(ASTFunction<'a>),
    Expression(ASTExpression<'a>),
}

#[derive(Debug)]
pub struct ASTExpression<'a> {
    pub left: Box<ASTValue<'a>>,
    pub right: Box<ASTValue<'a>>,
    pub op: ASTOperation,
}

#[derive(Debug)]
pub enum ASTOperation {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    And,
    Or,
    Not,
    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    BitwiseXor,
    BitwiseShiftLeft,
    BitwiseShiftRight,
    Plus,
    Minus,
    Multiply,
    Power,
    Divide,
    Remainder,
    TypeCast, // x as y
    TypeCheck, // x is y
    Contains, // x in y
    Await,
    PropertyAccess, // x.y
    Range,
}

#[derive(Debug)]
pub enum ASTStatement<'a> {
    Assert(ASTValue<'a>),
    /// (identifier, kind, value)
    Assignment(&'a str, ASTAssignmentKind, ASTValue<'a>),
    Break,
    Breakpoint,
    Continue,
    If(ASTValue<'a>, CodeBlock<'a>),
    Elif(ASTValue<'a>, CodeBlock<'a>),
    Else(ASTValue<'a>, CodeBlock<'a>),
    For(ASTVariable<'a>, ASTValue<'a>, CodeBlock<'a>),
    Pass,
    Return(ASTValue<'a>),
    /// (expression_being_matched_on, vec_of_patterns)
    Match(ASTValue<'a>, Vec<ASTMatchPattern<'a>>),
    While(ASTValue<'a>, CodeBlock<'a>),
    Variable(ASTVariable<'a>),
    Value(ASTValue<'a>),
}

#[derive(Debug)]
pub enum ASTAssignmentKind {
    Regular,
    Plus,
    Minus,
    Multiply,
    Power,
    Divide,
    Remainder,
    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    BitwiseXor,
    BitwiseShiftLeft,
    BitwiseShiftRight,
}

#[derive(Debug)]
pub struct ASTMatchPattern<'a> {
    pub kind: ASTMatchPatternKind<'a>,
}

#[derive(Debug)]
pub enum ASTMatchPatternKind<'a> {
    Expression(ASTValue<'a>),
    Binding(ASTVariable<'a>),
    Array(Vec<ASTMatchPattern<'a>>),
    Multiple(Vec<ASTMatchPattern<'a>>),
    // TODO: Dictionary patterns
}