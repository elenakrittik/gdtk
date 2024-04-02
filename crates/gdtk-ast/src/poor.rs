//! "Poor" GDScript abstract syntax tree. Does not interlink any references
//! and instead represents them as raw strings.

pub type CodeBlock<'a> = Vec<ASTStatement<'a>>;
pub type DictValue<'a> = Vec<(ASTValue<'a>, ASTValue<'a>)>;

#[derive(Debug, Clone, PartialEq)]
pub struct ASTFile<'a> {
    pub body: CodeBlock<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTClass<'a> {
    pub identifier: &'a str,
    pub extends: Option<&'a str>,
    pub body: CodeBlock<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTVariable<'a> {
    pub identifier: &'a str,
    pub infer_type: bool,
    pub typehint: Option<ASTValue<'a>>,
    pub value: Option<ASTValue<'a>>,
    pub kind: ASTVariableKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTVariableKind {
    /// Regular (`var`) variable.
    Regular,

    /// Constant (`const`) variable.
    Constant,

    /// Static (`static var`) variable.
    Static,

    /// A variable that represents a function parameter.
    FunctionParameter,

    /// A variable that represents a value bind in a match pattern.
    PatternBinding,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTEnum<'a> {
    pub identifier: Option<&'a str>,
    pub variants: Vec<ASTEnumVariant<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTEnumVariant<'a> {
    pub identifier: &'a str,
    pub value: Option<ASTValue<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTFunction<'a> {
    pub identifier: Option<&'a str>,
    pub parameters: Vec<ASTVariable<'a>>,
    pub return_type: Option<Box<ASTValue<'a>>>,
    pub body: CodeBlock<'a>,
}

#[derive(Debug, Clone, PartialEq, enum_as_inner::EnumAsInner)]
pub enum ASTValue<'a> {
    Identifier(&'a str),
    Number(i64),
    Float(f64),
    String(&'a str),
    StringName(&'a str),
    Node(&'a str),
    UniqueNode(&'a str),
    NodePath(&'a str),
    Boolean(bool),
    Array(Vec<ASTValue<'a>>),
    Dictionary(DictValue<'a>),
    Lambda(ASTFunction<'a>),
    UnaryExpr(ASTUnaryOp, Box<ASTValue<'a>>),
    /// (left, op, right)
    BinaryExpr(Box<ASTValue<'a>>, ASTBinaryOp, Box<ASTValue<'a>>),
    Comment(&'a str),
    Group(Box<ASTValue<'a>>),
}

impl<'a> prec::Token<ASTValue<'a>, ()> for ASTValue<'a> {
    fn convert(self, _ctx: &()) -> Result<ASTValue<'a>, ()> {
        Ok(self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum ASTUnaryOp {
    Await,
    Plus,
    Minus,
    Not,
    BitwiseNot,
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
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
    Range,          // ".." match pattern
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
    Call,
    Subscript,
    NotContains, // don't punch me for grammar
}

#[derive(Debug, Clone, PartialEq, enum_as_inner::EnumAsInner)]
pub enum ASTStatement<'a> {
    Annotation(ASTAnnotation<'a>),
    Assert(ASTValue<'a>),
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
    For(ASTForStmt<'a>),
    Func(ASTFunction<'a>),
    Pass,
    Return(ASTValue<'a>),
    Signal(ASTSignal<'a>),
    Match(ASTMatchStmt<'a>),
    While(ASTWhileStmt<'a>),
    Variable(ASTVariable<'a>),
    Value(ASTValue<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTForStmt<'a> {
    pub binding: &'a str,
    pub typehint: Option<ASTValue<'a>>,
    pub container: ASTValue<'a>,
    pub block: CodeBlock<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTWhileStmt<'a> {
    pub expr: ASTValue<'a>,
    pub block: CodeBlock<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTMatchStmt<'a> {
    pub expr: ASTValue<'a>,
    pub arms: Vec<ASTMatchArm<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTMatchArm<'a> {
    pub body: CodeBlock<'a>,
    pub kind: ASTMatchPattern<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTMatchPattern<'a> {
    Value(ASTValue<'a>),
    Binding(ASTVariable<'a>),
    Array(Vec<ASTMatchPattern<'a>>),
    // TODO: Dictionary(???),
    Alternative(Vec<ASTMatchPattern<'a>>),
    Rest,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTAnnotation<'a> {
    pub identifier: &'a str,
    pub arguments: Vec<ASTValue<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTSignal<'a> {
    pub identifier: &'a str,
    pub parameters: Vec<ASTVariable<'a>>,
}
