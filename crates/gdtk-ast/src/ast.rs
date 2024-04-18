//! "Poor" GDScript abstract syntax tree. Does not interlink any references
//! and instead represents them as raw strings.

/// A block of statements.
pub type CodeBlock<'a> = Vec<ASTStatement<'a>>;
pub type DictValue<'a> = (ASTExpr<'a>, ASTExpr<'a>);
pub type DictPattern<'a> = (ASTExpr<'a>, Option<Box<ASTMatchPattern<'a>>>);

type Range = std::ops::Range<usize>;

#[derive(Debug, Clone, PartialEq)]
pub struct ASTFile<'a> {
    pub body: CodeBlock<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTClass<'a> {
    pub identifier: ASTExpr<'a>,
    pub extends: Option<ASTExpr<'a>>,
    pub body: CodeBlock<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTVariable<'a> {
    pub identifier: ASTExpr<'a>,
    pub infer_type: bool,
    pub typehint: Option<ASTExpr<'a>>,
    pub value: Option<ASTExpr<'a>>,
    pub kind: ASTVariableKind,
}

impl<'a> ASTVariable<'a> {
    /// Creates a [ASTVariableKind::Binding] variable with ``infer_type: true``.
    pub fn new_binding(identifier: &'a str, range: Range) -> Self {
        Self {
            identifier: ASTExpr {
                range,
                kind: ASTExprKind::Identifier(identifier),
            },
            infer_type: true,
            typehint: None,
            value: None,
            kind: ASTVariableKind::Binding,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTVariableKind {
    /// Regular (`var`) variable.
    Regular,

    /// Constant (`const`) variable.
    Constant,

    /// Static (`static var`) variable.
    Static,

    /// A variable that represents a value binded by other means, like for loops,
    /// match patterns, and function parameters.
    Binding,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTEnum<'a> {
    pub identifier: Option<ASTExpr<'a>>,
    pub variants: Vec<ASTEnumVariant<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTEnumVariant<'a> {
    pub identifier: ASTExpr<'a>,
    pub value: Option<ASTExpr<'a>>,
}

/// A function.
#[derive(Debug, Clone, PartialEq)]
pub struct ASTFunction<'a> {
    pub identifier: Option<Box<ASTExpr<'a>>>,
    pub parameters: Vec<ASTVariable<'a>>,
    pub return_type: Option<Box<ASTExpr<'a>>>,
    pub body: CodeBlock<'a>,
}

/// An expression.
#[derive(Debug, Clone, PartialEq)]
pub struct ASTExpr<'a> {
    pub kind: ASTExprKind<'a>,
    pub range: Range,
}

/// An expression's kind.
#[derive(Debug, Clone, PartialEq, enum_as_inner::EnumAsInner)]
pub enum ASTExprKind<'a> {
    /// A parenthesized expression.
    Group(Vec<ASTExpr<'a>>),
    /// An identifier literal.
    Identifier(&'a str),
    /// An integer number literal.
    Number(u64),
    /// A float number literal.
    Float(f64),
    /// A string literal.
    String(&'a str),
    /// A ``StringName`` literal.
    StringName(&'a str),
    /// A ``Node`` literal.
    Node(&'a str),
    /// A ``UniqueNode`` literal.
    UniqueNode(&'a str),
    /// A ``NodePath`` literal.
    NodePath(&'a str),
    /// A boolean literal.
    Boolean(bool),
    /// A null literal.
    Null,
    /// An array literal.
    Array(Vec<ASTExpr<'a>>),
    /// A dictionary literal.
    Dictionary(Vec<DictValue<'a>>),
    /// A lambda function expression.
    Lambda(ASTFunction<'a>),
    /// An unary prefix expression.
    PrefixExpr(ASTPrefixOp, Box<ASTExpr<'a>>),
    /// An unary postfix expression.
    PostfixExpr(Box<ASTExpr<'a>>, ASTPostfixOp<'a>),
    /// A binary expression.
    BinaryExpr(Box<ASTExpr<'a>>, ASTBinaryOp<'a>, Box<ASTExpr<'a>>),
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, enum_as_inner::EnumAsInner)]
pub enum ASTPrefixOp {
    /// ``await a``.
    Await,
    /// ``+a``.
    Identity,
    /// ``-a``.
    Negation,
    /// ``not a`` or ``!a``.
    Not,
    /// ``~a``.
    BitwiseNot,
}

#[derive(Debug, Clone, PartialEq, enum_as_inner::EnumAsInner)]
pub enum ASTPostfixOp<'a> {
    /// ``a(b)``.
    Call(Vec<ASTExpr<'a>>),
    /// ``a[b]``.
    Subscript(Vec<ASTExpr<'a>>),
}

#[derive(Debug, Clone, PartialEq, enum_as_inner::EnumAsInner)]
pub enum ASTBinaryOp<'a> {
    /// ``a < b``.
    LessThan,
    /// ``a <= b``.
    LessOrEqual,
    /// ``a > b``.
    Greater,
    /// ``a >= b``.
    GreaterOrEqual,
    /// ``a == b``.
    Equals,
    /// ``a != b``.
    NotEqual,
    /// ``a and b`` or ``a && b``.
    And,
    /// ``a or b`` or ``a || b``.
    Or,
    /// ``a & b``.
    BitwiseAnd,
    /// ``a | b``.
    BitwiseOr,
    /// ``a ^ b``.
    BitwiseXor,
    /// ``a << b``.
    BitwiseShiftLeft,
    /// ``a >> b``.
    BitwiseShiftRight,
    /// ``a + b``.
    Add,
    /// ``a - b``.
    Subtract,
    /// ``a * b``.
    Multiply,
    /// ``a ** b``.
    Power,
    /// ``a / b``.
    Divide,
    /// ``a % b``.
    Remainder,
    /// ``a as b``.
    TypeCast,
    /// ``a is b``.
    TypeCheck,
    /// ``a in b``.
    Contains,
    /// ``a not in b``.
    NotContains, // don't punch me for grammar
    /// ``a.b``.
    PropertyAccess,
    /// ``a..b``. **UNOFFICIAL EXTENSION**.
    Range,
    /// ``a if c else b``.
    TernaryIfElse(Box<ASTExpr<'a>>),
    /// A placeholder for [ASTBinaryOp::TernaryIfElse].
    TernaryIfElsePlaceholder,
    /// ``a = b``.
    Assignment,
    /// ``a += b``.
    PlusAssignment,
    /// ``a -= b``.
    MinusAssignment,
    /// ``a *= b``.
    MultiplyAssignment,
    /// ``a **= b``.
    PowerAssignment,
    /// ``a /= b``.
    DivideAssignment,
    /// ``a %= b``.
    RemainderAssignment,
    /// ``a &= b``.
    BitwiseAndAssignment,
    /// ``a |= b``.
    BitwiseOrAssignment,
    /// ``a ~= b``.
    BitwiseNotAssignment,
    /// ``a ^= b``.
    BitwiseXorAssignment,
    /// ``a <<= b``.
    BitwiseShiftLeftAssignment,
    /// ``a >>= b``.
    BitwiseShiftRightAssignment,
}

/// A statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ASTStatement<'a> {
    pub kind: ASTStatementKind<'a>,
    pub range: Range,
}

/// A statement kind.
#[derive(Debug, Clone, PartialEq, enum_as_inner::EnumAsInner)]
pub enum ASTStatementKind<'a> {
    /// An annotation in form of a statement.
    Annotation(ASTAnnotation<'a>),
    /// An ``assert`` statement.
    Assert(ASTExpr<'a>),
    /// A ``break`` statement.
    Break,
    /// A ``breakpoint`` statement.
    Breakpoint,
    /// An inner class statement.
    Class(ASTClass<'a>),
    /// A ``class_name`` statement.
    ClassName(ASTExpr<'a>),
    /// A ``continue`` statement.
    Continue,
    /// An ``if`` statement.
    If(ASTIfStmt<'a>),
    /// An ``elif`` statement.
    Elif(ASTElifStmt<'a>),
    /// An ``else`` statement.
    Else(ASTElseStmt<'a>),
    /// A enum definition statement.
    Enum(ASTEnum<'a>),
    /// An ``extends`` statement.
    Extends(ASTExpr<'a>),
    /// A ``for`` loop statement.
    For(ASTForStmt<'a>),
    /// A function definition statement.
    Func(ASTFunction<'a>),
    /// A ``pass`` statement.
    Pass,
    /// A ``return`` statement.
    Return(Option<ASTExpr<'a>>),
    /// A ``signal`` definition statement.
    Signal(ASTSignal<'a>),
    /// A ``match`` statement.
    Match(ASTMatchStmt<'a>),
    /// A ``while`` loop statement.
    While(ASTWhileStmt<'a>),
    /// A variable definition statement.
    Variable(ASTVariable<'a>),
    /// A standalone expression.
    Expr(ASTExpr<'a>),
}

/// A ``for`` loop statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ASTForStmt<'a> {
    pub binding: ASTVariable<'a>,
    pub container: ASTExpr<'a>,
    pub block: CodeBlock<'a>,
}

/// A ``while`` loop statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ASTWhileStmt<'a> {
    pub expr: ASTExpr<'a>,
    pub block: CodeBlock<'a>,
}

/// An ``if`` statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ASTIfStmt<'a> {
    pub expr: ASTExpr<'a>,
    pub block: CodeBlock<'a>,
}

/// An ``elif`` statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ASTElifStmt<'a> {
    pub expr: ASTExpr<'a>,
    pub block: CodeBlock<'a>,
}

/// An ``else`` statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ASTElseStmt<'a> {
    pub block: CodeBlock<'a>,
}

/// A ``match`` statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ASTMatchStmt<'a> {
    pub expr: ASTExpr<'a>,
    pub arms: Vec<ASTMatchArm<'a>>,
}

/// An arm of a [ASTMatchStmt].
#[derive(Debug, Clone, PartialEq)]
pub struct ASTMatchArm<'a> {
    pub pattern: ASTMatchPattern<'a>,
    pub guard: Option<ASTExpr<'a>>,
    pub block: CodeBlock<'a>,
}

/// A pattern of an [ASTMatchArm].
#[derive(Debug, Clone, PartialEq)]
pub enum ASTMatchPattern<'a> {
    Value(ASTExpr<'a>),
    Binding(ASTVariable<'a>),
    Array(Vec<ASTMatchPattern<'a>>),
    Alternative(Vec<ASTMatchPattern<'a>>),
    Dictionary(Vec<DictPattern<'a>>),
    /// Represents the ".." found inside array and dictionary patterns.
    Ignore,
}

/// An ``@annotation`` attached to an item.
#[derive(Debug, Clone, PartialEq)]
pub struct ASTAnnotation<'a> {
    pub identifier: ASTExpr<'a>,
    pub arguments: Option<Vec<ASTExpr<'a>>>,
}

/// A ``signal`` definition.
#[derive(Debug, Clone, PartialEq)]
pub struct ASTSignal<'a> {
    pub identifier: ASTExpr<'a>,
    pub parameters: Option<Vec<ASTVariable<'a>>>,
}
