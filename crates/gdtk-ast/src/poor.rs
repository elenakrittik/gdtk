//! "Poor" GDScript abstract syntax tree. Does not interlink any references
//! and instead represents them as raw strings.

pub type CodeBlock<'a> = Vec<ASTStatement<'a>>;
pub type DictValue<'a> = Vec<(ASTValue<'a>, ASTValue<'a>)>;

#[derive(Debug, Clone)]
pub struct ASTFile<'a> {
    pub body: CodeBlock<'a>,
}

#[derive(Debug, Clone)]
pub struct ASTClass<'a> {
    pub name: Option<&'a str>,
    pub extends: Option<&'a str>,
    pub variables: Vec<ASTVariable<'a>>,
    pub enums: Vec<ASTEnum<'a>>,
    pub functions: Vec<ASTFunction<'a>>,
    pub signals: Vec<ASTSignal<'a>>,
    pub inner_classes: Vec<ASTClass<'a>>,
}

#[derive(Debug, Clone)]
pub struct ASTVariable<'a> {
    pub identifier: &'a str,
    pub infer_type: bool,
    pub typehint: Option<ASTValue<'a>>,
    pub value: Option<ASTValue<'a>>,
    pub kind: ASTVariableKind,
}

#[derive(Debug, Clone)]
pub enum ASTVariableKind {
    /// Regular (`var`) variable.
    Regular,

    /// Constant (`const`) variable.
    Constant,

    /// Static (`static var`) variable.
    Static,

    /// A variable that represents a function parameter.
    FunctionParameter,
}

#[derive(Debug, Clone)]
pub struct ASTEnum<'a> {
    pub identifier: Option<&'a str>,
    pub variants: Vec<ASTEnumVariant<'a>>,
}

#[derive(Debug, Clone)]
pub struct ASTEnumVariant<'a> {
    pub identifier: &'a str,
    pub value: Option<ASTValue<'a>>,
}

#[derive(Debug, Clone)]
pub struct ASTFunction<'a> {
    pub identifier: Option<&'a str>,
    pub parameters: Vec<ASTVariable<'a>>,
    pub return_type: Option<Box<ASTValue<'a>>>,
    pub body: CodeBlock<'a>,
}

#[derive(Debug, Clone, enum_as_inner::EnumAsInner)]
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
    Dictionary(DictValue<'a>),
    Lambda(ASTFunction<'a>),
    UnaryExpr(ASTUnaryOp, Box<ASTValue<'a>>),
    /// (left, op, right)
    BinaryExpr(Box<ASTValue<'a>>, ASTBinaryOp, Box<ASTValue<'a>>),
    /// (function_expr, arguments)
    Call(Box<ASTValue<'a>>, Vec<ASTValue<'a>>),
    /// (subcript_expr, index)
    Subscript(Box<ASTValue<'a>>, Box<ASTValue<'a>>),
    Comment(&'a str),
}

#[derive(Debug, Clone)]
pub enum ASTUnaryOp {
    Await,
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
pub enum ASTBinaryOp {
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
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
    TypeCast,       // x as y
    TypeCheck,      // x is y
    Contains,       // x in y
    PropertyAccess, // x.y
    Range,          // x..y // TODO: rename to "Rest"?
}

#[derive(Debug, Clone, enum_as_inner::EnumAsInner)]
pub enum ASTStatement<'a> {
    Annotation(ASTAnnotation<'a>),
    Assert(ASTValue<'a>),
    /// (identifier, kind, value)
    Assignment(&'a str, ASTAssignmentKind, ASTValue<'a>),
    Break,
    Breakpoint,
    Class(ASTClass<'a>),
    ClassName(&'a str),
    Continue,
    If(ASTValue<'a>, CodeBlock<'a>),
    Elif(ASTValue<'a>, CodeBlock<'a>),
    Else(CodeBlock<'a>),
    Enum(ASTEnum<'a>),
    Extends(&'a str),
    /// (identifier, type_hint, container, body)
    For(&'a str, Option<ASTValue<'a>>, ASTValue<'a>, CodeBlock<'a>),
    Func(ASTFunction<'a>),
    Pass,
    Return(ASTValue<'a>),
    /// (name, args)
    Signal(ASTSignal<'a>),
    /// (expression_being_matched_on, vec_of_patterns)
    Match(ASTValue<'a>, Vec<ASTMatchPattern<'a>>),
    While(ASTValue<'a>, CodeBlock<'a>),
    Variable(ASTVariable<'a>),
    Value(ASTValue<'a>),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct ASTMatchPattern<'a> {
    pub body: CodeBlock<'a>,
    pub kind: ASTMatchPatternKind<'a>,
}

#[derive(Debug, Clone)]
pub enum ASTMatchPatternKind<'a> {
    Value(ASTValue<'a>),
    Binding(ASTVariable<'a>),
    Array(Vec<ASTMatchPatternKind<'a>>),
    // TODO: Dictionary(???),
    Alternative(Vec<ASTMatchPatternKind<'a>>),
    Rest,
}

#[derive(Debug, Clone)]
pub struct ASTAnnotation<'a> {
    pub identifier: &'a str,
    pub arguments: Vec<ASTValue<'a>>,
}

#[derive(Debug, Clone)]
pub struct ASTSignal<'a> {
    pub identifier: &'a str,
    pub parameters: Vec<ASTVariable<'a>>,
}
