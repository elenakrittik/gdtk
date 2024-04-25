//! A bare-bones GDScript abstract syntax tree. All references are plain strings.
//!
//! Note that not all nodes have spans attached to them yet, this will be resolved
//! as needs arise.

use gdtk_span::Span;

/// A block of statements.
pub type CodeBlock<'a> = Vec<ASTStatement<'a>>;
pub type DictValue<'a> = (ASTExpr<'a>, ASTExpr<'a>);
pub type DictPattern<'a> = (ASTExpr<'a>, Option<Box<ASTMatchPattern<'a>>>);

#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTFile<'a> {
    pub body: CodeBlock<'a>,
}

#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTClassStmt<'a> {
    pub identifier: ASTExpr<'a>,
    pub extends: Option<ASTExpr<'a>>,
    pub body: CodeBlock<'a>,
}

#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTVariable<'a> {
    pub identifier: ASTExpr<'a>,
    pub infer_type: bool,
    pub typehint: Option<ASTExpr<'a>>,
    pub value: Option<ASTExpr<'a>>,
    pub kind: ASTVariableKind,
    pub getter: Option<ASTFunction<'a>>,
    pub setter: Option<ASTFunction<'a>>,
}

impl<'a> ASTVariable<'a> {
    /// Creates a [ASTVariableKind::Binding] variable with ``infer_type: true``.
    pub fn new_binding(identifier: ASTExpr<'a>) -> Self {
        Self {
            identifier,
            infer_type: true,
            typehint: None,
            value: None,
            kind: ASTVariableKind::Binding,
            getter: None,
            setter: None,
        }
    }
}

#[derive(Debug, Clone, derivative::Derivative, enum_as_inner::EnumAsInner)]
#[derivative(PartialEq)]
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

#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTEnumStmt<'a> {
    pub identifier: Option<ASTExpr<'a>>,
    pub variants: Vec<ASTEnumVariant<'a>>,
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
}

#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTEnumVariant<'a> {
    pub identifier: ASTExpr<'a>,
    pub value: Option<ASTExpr<'a>>,
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
}

/// A function.
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTFunction<'a> {
    pub identifier: Option<Box<ASTExpr<'a>>>,
    pub parameters: Vec<ASTVariable<'a>>,
    pub return_type: Option<Box<ASTExpr<'a>>>,
    pub kind: ASTFunctionKind,
    pub body: CodeBlock<'a>,
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
}

/// A function's kind.
#[derive(Debug, Copy, Clone, PartialEq, enum_as_inner::EnumAsInner)]
pub enum ASTFunctionKind {
    /// A normal `func`.
    Regular,
    /// A `static func`.
    Static,
}

/// An expression.
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTExpr<'a> {
    pub kind: ASTExprKind<'a>,
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
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

#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTPrefixOp {
    pub kind: ASTPrefixOpKind,
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, enum_as_inner::EnumAsInner)]
pub enum ASTPrefixOpKind {
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

#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTPostfixOp<'a> {
    pub kind: ASTPostfixOpKind<'a>,
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, enum_as_inner::EnumAsInner)]
pub enum ASTPostfixOpKind<'a> {
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

impl ASTBinaryOp<'_> {
    pub fn is_any_assignment(&self) -> bool {
        matches!(
            self,
            ASTBinaryOp::Assignment
                | ASTBinaryOp::PlusAssignment
                | ASTBinaryOp::MinusAssignment
                | ASTBinaryOp::MultiplyAssignment
                | ASTBinaryOp::PowerAssignment
                | ASTBinaryOp::DivideAssignment
                | ASTBinaryOp::RemainderAssignment
                | ASTBinaryOp::BitwiseAndAssignment
                | ASTBinaryOp::BitwiseOrAssignment
                | ASTBinaryOp::BitwiseNotAssignment
                | ASTBinaryOp::BitwiseXorAssignment
                | ASTBinaryOp::BitwiseShiftLeftAssignment
                | ASTBinaryOp::BitwiseShiftRightAssignment
        )
    }
}

/// A statement.
#[derive(Debug, Clone, PartialEq, enum_as_inner::EnumAsInner)]
pub enum ASTStatement<'a> {
    /// An annotation in form of a statement.
    Annotation(ASTAnnotationStmt<'a>),
    /// An ``assert`` statement.
    Assert(ASTAssertStmt<'a>),
    /// A ``break`` statement.
    Break(ASTBreakStmt),
    /// A ``breakpoint`` statement.
    Breakpoint(ASTBreakpointStmt),
    /// An inner class statement.
    Class(ASTClassStmt<'a>),
    /// A ``class_name`` statement.
    ClassName(ASTClassNameStmt<'a>),
    /// A ``continue`` statement.
    Continue(ASTContinueStmt),
    /// An ``if`` statement.
    If(ASTIfStmt<'a>),
    /// An ``elif`` statement.
    Elif(ASTElifStmt<'a>),
    /// An ``else`` statement.
    Else(ASTElseStmt<'a>),
    /// A enum definition statement.
    Enum(ASTEnumStmt<'a>),
    /// An ``extends`` statement.
    Extends(ASTExtendsStmt<'a>),
    /// A ``for`` loop statement.
    For(ASTForStmt<'a>),
    /// A function definition statement.
    Func(ASTFunction<'a>),
    /// A ``pass`` statement.
    Pass(ASTPassStmt),
    /// A ``return`` statement.
    Return(ASTReturnStmt<'a>),
    /// A ``signal`` definition statement.
    Signal(ASTSignalStmt<'a>),
    /// A ``match`` statement.
    Match(ASTMatchStmt<'a>),
    /// A ``while`` loop statement.
    While(ASTWhileStmt<'a>),
    /// A variable definition statement.
    Variable(ASTVariable<'a>),
    /// A standalone expression.
    Expr(ASTExpr<'a>),
}

impl ASTStatement<'_> {
    /// The range of the statement. FIXME: make this always return a range
    pub fn range(&self) -> Option<&Span> {
        match self {
            ASTStatement::Annotation(_stmt) => None,
            ASTStatement::Assert(stmt) => Some(&stmt.span),
            ASTStatement::Break(stmt) => Some(&stmt.span),
            ASTStatement::Breakpoint(stmt) => Some(&stmt.span),
            ASTStatement::Class(_stmt) => None,
            ASTStatement::ClassName(stmt) => Some(&stmt.span),
            ASTStatement::Continue(stmt) => Some(&stmt.span),
            ASTStatement::If(_stmt) => None,
            ASTStatement::Elif(_stmt) => None,
            ASTStatement::Else(_stmt) => None,
            ASTStatement::Enum(stmt) => Some(&stmt.span),
            ASTStatement::Extends(stmt) => Some(&stmt.span),
            ASTStatement::For(_stmt) => None,
            ASTStatement::Func(stmt) => Some(&stmt.span),
            ASTStatement::Pass(stmt) => Some(&stmt.span),
            ASTStatement::Return(stmt) => Some(&stmt.span),
            ASTStatement::Signal(_stmt) => None,
            ASTStatement::Match(_stmt) => None,
            ASTStatement::While(_stmt) => None,
            ASTStatement::Variable(_stmt) => None,
            ASTStatement::Expr(stmt) => Some(&stmt.span),
        }
    }
}

/// A pass statement.
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTPassStmt {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
}

/// An ``assert`` statement.
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTAssertStmt<'a> {
    pub expr: ASTExpr<'a>,
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
}

/// A ``break`` statement.
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTBreakStmt {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
}

/// A ``breakpoint`` statement.
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTBreakpointStmt {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
}

/// A ``class_name`` statement.
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTClassNameStmt<'a> {
    pub identifier: ASTExpr<'a>,
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
}

/// A ``continue`` statement.
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTContinueStmt {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
}

/// An ``extends`` statement.
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTExtendsStmt<'a> {
    pub identifier: ASTExpr<'a>,
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
}

/// A ``for`` loop statement.
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTForStmt<'a> {
    pub binding: ASTVariable<'a>,
    pub container: ASTExpr<'a>,
    pub block: CodeBlock<'a>,
}

/// A return statement.
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTReturnStmt<'a> {
    pub expr: Option<ASTExpr<'a>>,
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
}

/// A ``while`` loop statement.
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTWhileStmt<'a> {
    pub expr: ASTExpr<'a>,
    pub block: CodeBlock<'a>,
}

/// An ``if`` statement.
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTIfStmt<'a> {
    pub expr: ASTExpr<'a>,
    pub block: CodeBlock<'a>,
}

/// An ``elif`` statement.
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTElifStmt<'a> {
    pub expr: ASTExpr<'a>,
    pub block: CodeBlock<'a>,
}

/// An ``else`` statement.
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTElseStmt<'a> {
    pub block: CodeBlock<'a>,
}

/// A ``match`` statement.
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTMatchStmt<'a> {
    pub expr: ASTExpr<'a>,
    pub arms: Vec<ASTMatchArm<'a>>,
}

/// An arm of a [ASTMatchStmt].
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTMatchArm<'a> {
    pub pattern: ASTMatchPattern<'a>,
    pub guard: Option<ASTExpr<'a>>,
    pub block: CodeBlock<'a>,
}

/// A pattern of an [ASTMatchArm].
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
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
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTAnnotationStmt<'a> {
    pub identifier: ASTExpr<'a>,
    pub arguments: Option<Vec<ASTExpr<'a>>>,
}

/// A ``signal`` definition.
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct ASTSignalStmt<'a> {
    pub identifier: ASTExpr<'a>,
    pub parameters: Option<Vec<ASTVariable<'a>>>,
}
