use crate::ast;

use gdtk_span::Span;

pub trait Visitor<'a>: Sized {
    fn visit_file(&mut self, file: &'a ast::ASTFile<'a>) {
        walk_file(self, file);
    }

    fn visit_block(&mut self, block: &'a [ast::ASTStatement<'a>]) {
        walk_block(self, block);
    }

    fn visit_statement(&mut self, stmt: &'a ast::ASTStatement<'a>) {
        walk_statement(self, stmt);
    }

    fn visit_annotation(&mut self, ann: &'a ast::ASTAnnotationStmt<'a>) {
        walk_annotation(self, ann);
    }

    fn visit_assert_statement(&mut self, stmt: &'a ast::ASTAssertStmt<'a>) {
        walk_assert_statement(self, stmt);
    }

    fn visit_break_statement(&mut self, stmt: &'a ast::ASTBreakStmt) {
        walk_break_statement(self, stmt);
    }

    fn visit_breakpoint_statement(&mut self, stmt: &'a ast::ASTBreakpointStmt) {
        walk_breakpoint_statement(self, stmt);
    }

    fn visit_class(&mut self, class: &'a ast::ASTClassStmt<'a>) {
        walk_class(self, class);
    }

    fn visit_class_name_statement(&mut self, stmt: &'a ast::ASTClassNameStmt<'a>) {
        walk_class_name_statement(self, stmt);
    }

    fn visit_continue_statement(&mut self, stmt: &'a ast::ASTContinueStmt) {
        walk_continue_statement(self, stmt);
    }

    fn visit_if_statement(&mut self, stmt: &'a ast::ASTIfStmt<'a>) {
        walk_if_statement(self, stmt);
    }

    fn visit_elif_statement(&mut self, stmt: &'a ast::ASTElifStmt<'a>) {
        walk_elif_statement(self, stmt);
    }

    fn visit_else_statement(&mut self, stmt: &'a ast::ASTElseStmt<'a>) {
        walk_else_statement(self, stmt);
    }

    fn visit_enum_statement(&mut self, enum_: &'a ast::ASTEnumStmt<'a>) {
        walk_enum_statement(self, enum_);
    }

    fn visit_enum_variants(&mut self, variants: &'a [ast::ASTEnumVariant<'a>]) {
        walk_enum_variants(self, variants);
    }

    fn visit_enum_variant(&mut self, variant: &'a ast::ASTEnumVariant<'a>) {
        walk_enum_variant(self, variant);
    }

    fn visit_extends_statement(&mut self, stmt: &'a ast::ASTExtendsStmt<'a>) {
        walk_extends_statement(self, stmt);
    }

    fn visit_for_statement(&mut self, stmt: &'a ast::ASTForStmt<'a>) {
        walk_for_statement(self, stmt);
    }

    fn visit_func(&mut self, func: &'a ast::ASTFunction<'a>) {
        walk_func(self, func);
    }

    fn visit_parameters(&mut self, parameters: &'a [ast::ASTVariable<'a>]) {
        walk_parameters(self, parameters);
    }

    fn visit_pass_statement(&mut self, stmt: &'a ast::ASTPassStmt) {
        walk_pass_statement(self, stmt);
    }

    fn visit_return_statement(&mut self, stmt: &'a ast::ASTReturnStmt<'a>) {
        walk_return_statement(self, stmt);
    }

    fn visit_signal_statement(&mut self, signal: &'a ast::ASTSignalStmt<'a>) {
        walk_signal_statement(self, signal);
    }

    fn visit_match_statement(&mut self, stmt: &'a ast::ASTMatchStmt<'a>) {
        walk_match_statement(self, stmt);
    }

    fn visit_match_arms(&mut self, arms: &'a [ast::ASTMatchArm<'a>]) {
        walk_match_arms(self, arms);
    }

    fn visit_match_arm(&mut self, arm: &'a ast::ASTMatchArm<'a>) {
        walk_match_arm(self, arm);
    }

    fn visit_match_patterns(&mut self, patterns: &'a [ast::ASTMatchPattern<'a>]) {
        walk_match_patterns(self, patterns);
    }

    fn visit_match_pattern(&mut self, pattern: &'a ast::ASTMatchPattern<'a>) {
        walk_match_pattern(self, pattern);
    }

    fn visit_match_expr_pattern(&mut self, expr: &'a ast::ASTExpr<'a>) {
        walk_match_expr_pattern(self, expr);
    }

    fn visit_match_binding_pattern(&mut self, binding: &'a ast::ASTVariable<'a>) {
        walk_match_binding_pattern(self, binding);
    }

    fn visit_match_array_pattern(&mut self, subpatterns: &'a [ast::ASTMatchPattern<'a>]) {
        walk_match_array_pattern(self, subpatterns);
    }

    fn visit_match_dictionary_pattern(&mut self, subpatterns: &'a [ast::DictPattern<'a>]) {
        walk_match_dictionary_pattern(self, subpatterns);
    }

    fn visit_match_alternative_pattern(&mut self, subpatterns: &'a [ast::ASTMatchPattern<'a>]) {
        walk_match_alternative_pattern(self, subpatterns);
    }

    fn visit_match_ignore_pattern(&mut self) {
        walk_match_ignore_pattern(self);
    }

    fn visit_match_guard(&mut self, expr: &'a ast::ASTExpr<'a>) {
        walk_match_guard(self, expr);
    }

    fn visit_while_statement(&mut self, stmt: &'a ast::ASTWhileStmt<'a>) {
        walk_while_statement(self, stmt);
    }

    fn visit_variable(&mut self, variable: &'a ast::ASTVariable<'a>) {
        walk_variable(self, variable);
    }

    fn visit_regular_variable(&mut self, variable: &'a ast::ASTVariable<'a>) {
        walk_regular_variable(self, variable);
    }

    fn visit_const_variable(&mut self, variable: &'a ast::ASTVariable<'a>) {
        walk_const_variable(self, variable);
    }

    fn visit_static_variable(&mut self, variable: &'a ast::ASTVariable<'a>) {
        walk_static_variable(self, variable);
    }

    fn visit_binding_variable(&mut self, variable: &'a ast::ASTVariable<'a>) {
        walk_binding_variable(self, variable);
    }

    fn visit_any_variable(&mut self, variable: &'a ast::ASTVariable<'a>) {
        walk_any_variable(self, variable);
    }

    fn visit_exprs(&mut self, exprs: &'a [ast::ASTExpr<'a>]) {
        walk_exprs(self, exprs);
    }

    fn visit_expr(&mut self, expr: &'a ast::ASTExpr<'a>) {
        walk_expr(self, expr);
    }

    fn visit_group_expr(&mut self, exprs: &'a [ast::ASTExpr], span: &'a Span) {
        walk_group_expr(self, exprs, span)
    }

    fn visit_identifier_expr(&mut self, identifier: &'a str, span: &'a Span) {
        walk_identifier_expr(self, identifier, span)
    }

    fn visit_number_expr(&mut self, number: u64, span: &'a Span) {
        walk_number_expr(self, number, span)
    }

    fn visit_float_expr(&mut self, float: f64, span: &'a Span) {
        walk_float_expr(self, float, span)
    }

    fn visit_string_expr(&mut self, string: &'a str, span: &'a Span) {
        walk_string_expr(self, string, span)
    }

    fn visit_string_name_expr(&mut self, string: &'a str, span: &'a Span) {
        walk_string_name_expr(self, string, span)
    }

    fn visit_node_expr(&mut self, path: &'a str, span: &'a Span) {
        walk_node_expr(self, path, span)
    }

    fn visit_unique_node_expr(&mut self, path: &'a str, span: &'a Span) {
        walk_unique_node_expr(self, path, span)
    }

    fn visit_node_path_expr(&mut self, path: &'a str, span: &'a Span) {
        walk_node_path_expr(self, path, span)
    }

    fn visit_boolean_expr(&mut self, boolean: bool, span: &'a Span) {
        walk_boolean_expr(self, boolean, span)
    }

    fn visit_null_expr(&mut self, span: &'a Span) {
        walk_null_expr(self, span)
    }

    fn visit_array_expr(&mut self, exprs: &'a [ast::ASTExpr], span: &'a Span) {
        walk_array_expr(self, exprs, span)
    }

    fn visit_dictionary_expr(&mut self, pairs: &'a [ast::DictValue], span: &'a Span) {
        walk_dictionary_expr(self, pairs, span)
    }

    fn visit_lambda_expr(&mut self, func: &'a ast::ASTFunction<'a>) {
        walk_lambda_expr(self, func);
    }

    fn visit_prefix_expr(&mut self, op: &'a ast::ASTPrefixOp, expr: &'a ast::ASTExpr<'a>) {
        walk_prefix_expr(self, op, expr);
    }

    fn visit_postfix_expr(
        &mut self,
        expr: &'a ast::ASTExpr,
        op: &'a ast::ASTPostfixOp,
        span: &'a Span,
    ) {
        walk_postfix_expr(self, expr, op, span);
    }

    fn visit_binary_expr(
        &mut self,
        lhs: &'a ast::ASTExpr,
        op: &'a ast::ASTBinaryOp,
        rhs: &'a ast::ASTExpr,
        span: &'a Span,
    ) {
        walk_binary_expr(self, lhs, op, rhs, span)
    }
}

pub fn walk_file<'a>(visitor: &mut impl Visitor<'a>, file: &'a ast::ASTFile<'a>) {
    visitor.visit_block(&file.body);
}

pub fn walk_block<'a>(visitor: &mut impl Visitor<'a>, block: &'a [ast::ASTStatement<'a>]) {
    for stmt in block {
        visitor.visit_statement(stmt);
    }
}

#[rustfmt::skip]
pub fn walk_statement<'a>(visitor: &mut impl Visitor<'a>, stmt: &'a ast::ASTStatement<'a>) {
    match stmt {
        ast::ASTStatement::Annotation(stmt) => visitor.visit_annotation(stmt),
        ast::ASTStatement::Assert(stmt) => visitor.visit_assert_statement(stmt),
        ast::ASTStatement::Break(stmt) => visitor.visit_break_statement(stmt),
        ast::ASTStatement::Breakpoint(stmt) => visitor.visit_breakpoint_statement(stmt),
        ast::ASTStatement::Class(stmt) => visitor.visit_class(stmt),
        ast::ASTStatement::ClassName(stmt) => visitor.visit_class_name_statement(stmt),
        ast::ASTStatement::Continue(stmt) => visitor.visit_continue_statement(stmt),
        ast::ASTStatement::If(stmt) => visitor.visit_if_statement(stmt),
        ast::ASTStatement::Elif(stmt) => visitor.visit_elif_statement(stmt),
        ast::ASTStatement::Else(stmt) => visitor.visit_else_statement(stmt),
        ast::ASTStatement::Enum(stmt) => visitor.visit_enum_statement(stmt),
        ast::ASTStatement::Extends(stmt) => visitor.visit_extends_statement(stmt),
        ast::ASTStatement::For(stmt) => visitor.visit_for_statement(stmt),
        ast::ASTStatement::Func(func) => visitor.visit_func(func),
        ast::ASTStatement::Pass(stmt) => visitor.visit_pass_statement(stmt),
        ast::ASTStatement::Return(stmt) => visitor.visit_return_statement(stmt),
        ast::ASTStatement::Signal(stmt) => visitor.visit_signal_statement(stmt),
        ast::ASTStatement::Match(stmt) => visitor.visit_match_statement(stmt),
        ast::ASTStatement::While(stmt) => visitor.visit_while_statement(stmt),
        ast::ASTStatement::Variable(variable) => visitor.visit_variable(variable),
        ast::ASTStatement::Expr(expr) => visitor.visit_expr(expr),
    }
}

pub fn walk_annotation<'a>(visitor: &mut impl Visitor<'a>, ann: &'a ast::ASTAnnotationStmt<'a>) {
    visitor.visit_expr(&ann.identifier);

    if let Some(args) = &ann.arguments {
        visitor.visit_exprs(args.as_slice());
    }
}

pub fn walk_assert_statement<'a>(visitor: &mut impl Visitor<'a>, stmt: &'a ast::ASTAssertStmt<'a>) {
    visitor.visit_expr(&stmt.expr);
}

pub fn walk_break_statement<'a>(_visitor: &mut impl Visitor<'a>, _stmt: &'a ast::ASTBreakStmt) {}
pub fn walk_breakpoint_statement<'a>(
    _visitor: &mut impl Visitor<'a>,
    _stmt: &'a ast::ASTBreakpointStmt,
) {
}

pub fn walk_class<'a>(visitor: &mut impl Visitor<'a>, class: &'a ast::ASTClassStmt<'a>) {
    visitor.visit_expr(&class.identifier);

    if let Some(extends) = &class.extends {
        visitor.visit_expr(extends);
    }

    visitor.visit_block(class.body.as_slice());
}

pub fn walk_class_name_statement<'a>(
    visitor: &mut impl Visitor<'a>,
    stmt: &'a ast::ASTClassNameStmt<'a>,
) {
    visitor.visit_expr(&stmt.identifier);
}

pub fn walk_continue_statement<'a>(
    _visitor: &mut impl Visitor<'a>,
    _stmt: &'a ast::ASTContinueStmt,
) {
}

pub fn walk_if_statement<'a>(visitor: &mut impl Visitor<'a>, stmt: &'a ast::ASTIfStmt<'a>) {
    visitor.visit_expr(&stmt.expr);
    visitor.visit_block(stmt.block.as_slice());
}

pub fn walk_elif_statement<'a>(visitor: &mut impl Visitor<'a>, stmt: &'a ast::ASTElifStmt<'a>) {
    visitor.visit_expr(&stmt.expr);
    visitor.visit_block(stmt.block.as_slice());
}

pub fn walk_else_statement<'a>(visitor: &mut impl Visitor<'a>, stmt: &'a ast::ASTElseStmt<'a>) {
    visitor.visit_block(stmt.block.as_slice());
}

pub fn walk_enum_statement<'a>(visitor: &mut impl Visitor<'a>, enum_: &'a ast::ASTEnumStmt<'a>) {
    if let Some(identfier) = &enum_.identifier {
        visitor.visit_expr(identfier)
    }

    visitor.visit_enum_variants(enum_.variants.as_slice());
}

pub fn walk_enum_variants<'a>(
    visitor: &mut impl Visitor<'a>,
    variants: &'a [ast::ASTEnumVariant<'a>],
) {
    for variant in variants {
        visitor.visit_enum_variant(variant);
    }
}

pub fn walk_enum_variant<'a>(visitor: &mut impl Visitor<'a>, variant: &'a ast::ASTEnumVariant<'a>) {
    visitor.visit_expr(&variant.identifier);

    if let Some(expr) = &variant.value {
        visitor.visit_expr(expr);
    }
}

pub fn walk_extends_statement<'a>(
    visitor: &mut impl Visitor<'a>,
    stmt: &'a ast::ASTExtendsStmt<'a>,
) {
    visitor.visit_expr(&stmt.identifier)
}

pub fn walk_for_statement<'a>(visitor: &mut impl Visitor<'a>, stmt: &'a ast::ASTForStmt<'a>) {
    visitor.visit_variable(&stmt.binding);
    visitor.visit_expr(&stmt.container);
    visitor.visit_block(stmt.block.as_slice());
}

pub fn walk_func<'a>(visitor: &mut impl Visitor<'a>, func: &'a ast::ASTFunction<'a>) {
    if let Some(identifier) = &func.identifier {
        visitor.visit_expr(identifier)
    }

    visitor.visit_parameters(func.parameters.as_slice());

    if let Some(return_type) = &func.return_type {
        visitor.visit_expr(return_type);
    }

    visitor.visit_block(func.body.as_slice());
}

pub fn walk_parameters<'a>(visitor: &mut impl Visitor<'a>, parameters: &'a [ast::ASTVariable<'a>]) {
    for param in parameters {
        visitor.visit_variable(param);
    }
}

pub fn walk_pass_statement<'a>(_visitor: &mut impl Visitor<'a>, _stmt: &'a ast::ASTPassStmt) {}

pub fn walk_return_statement<'a>(visitor: &mut impl Visitor<'a>, stmt: &'a ast::ASTReturnStmt<'a>) {
    if let Some(expr) = &stmt.expr {
        visitor.visit_expr(expr);
    }
}

pub fn walk_signal_statement<'a>(
    visitor: &mut impl Visitor<'a>,
    signal: &'a ast::ASTSignalStmt<'a>,
) {
    visitor.visit_expr(&signal.identifier);

    if let Some(params) = &signal.parameters {
        visitor.visit_parameters(params.as_slice());
    }
}

pub fn walk_match_statement<'a>(visitor: &mut impl Visitor<'a>, stmt: &'a ast::ASTMatchStmt<'a>) {
    visitor.visit_expr(&stmt.expr);
    visitor.visit_match_arms(stmt.arms.as_slice());
}

pub fn walk_match_arms<'a>(visitor: &mut impl Visitor<'a>, arms: &'a [ast::ASTMatchArm<'a>]) {
    for arm in arms {
        visitor.visit_match_arm(arm);
    }
}

pub fn walk_match_arm<'a>(visitor: &mut impl Visitor<'a>, arm: &'a ast::ASTMatchArm<'a>) {
    visitor.visit_match_pattern(&arm.pattern);

    if let Some(guard) = &arm.guard {
        visitor.visit_match_guard(guard);
    }

    visitor.visit_block(arm.block.as_slice());
}

pub fn walk_match_patterns<'a>(
    visitor: &mut impl Visitor<'a>,
    patterns: &'a [ast::ASTMatchPattern<'a>],
) {
    for pattern in patterns {
        visitor.visit_match_pattern(pattern);
    }
}

#[rustfmt::skip]
pub fn walk_match_pattern<'a>(visitor: &mut impl Visitor<'a>, pattern: &'a ast::ASTMatchPattern<'a>) {
    match pattern {
        ast::ASTMatchPattern::Value(expr) => visitor.visit_expr(expr),
        ast::ASTMatchPattern::Binding(binding) => visitor.visit_match_binding_pattern(binding),
        ast::ASTMatchPattern::Array(subpatterns) => visitor.visit_match_array_pattern(subpatterns),
        ast::ASTMatchPattern::Dictionary(subpatterns) => visitor.visit_match_dictionary_pattern(subpatterns),
        ast::ASTMatchPattern::Alternative(subpatterns) => visitor.visit_match_alternative_pattern(subpatterns),
        ast::ASTMatchPattern::Ignore => visitor.visit_match_ignore_pattern(),
    }
}

pub fn walk_match_expr_pattern<'a>(visitor: &mut impl Visitor<'a>, expr: &'a ast::ASTExpr<'a>) {
    visitor.visit_expr(expr);
}

pub fn walk_match_binding_pattern<'a>(
    visitor: &mut impl Visitor<'a>,
    binding: &'a ast::ASTVariable<'a>,
) {
    visitor.visit_variable(binding);
}

pub fn walk_match_array_pattern<'a>(
    visitor: &mut impl Visitor<'a>,
    subpatterns: &'a [ast::ASTMatchPattern<'a>],
) {
    visitor.visit_match_patterns(subpatterns);
}

pub fn walk_match_dictionary_pattern<'a>(
    visitor: &mut impl Visitor<'a>,
    subpatterns: &'a [ast::DictPattern],
) {
    for (key, value) in subpatterns {
        visitor.visit_expr(key);

        if let Some(value) = value {
            visitor.visit_match_pattern(value);
        }
    }
}

pub fn walk_match_alternative_pattern<'a>(
    visitor: &mut impl Visitor<'a>,
    subpatterns: &'a [ast::ASTMatchPattern<'a>],
) {
    visitor.visit_match_patterns(subpatterns);
}

pub fn walk_match_ignore_pattern<'a>(_visitor: &mut impl Visitor<'a>) {}

pub fn walk_match_guard<'a>(visitor: &mut impl Visitor<'a>, expr: &'a ast::ASTExpr<'a>) {
    visitor.visit_expr(expr);
}

pub fn walk_while_statement<'a>(visitor: &mut impl Visitor<'a>, stmt: &'a ast::ASTWhileStmt<'a>) {
    visitor.visit_expr(&stmt.expr);
    visitor.visit_block(stmt.block.as_slice());
}

#[rustfmt::skip]
pub fn walk_variable<'a>(visitor: &mut impl Visitor<'a>, variable: &'a ast::ASTVariable<'a>) {
    match variable {
        ast::ASTVariable { kind: ast::ASTVariableKind::Regular, .. } => visitor.visit_regular_variable(variable),
        ast::ASTVariable { kind: ast::ASTVariableKind::Constant, .. } => visitor.visit_const_variable(variable),
        ast::ASTVariable { kind: ast::ASTVariableKind::Static, .. } => visitor.visit_static_variable(variable),
        ast::ASTVariable { kind: ast::ASTVariableKind::Binding, .. } => visitor.visit_binding_variable(variable),
    }
}

pub fn walk_regular_variable<'a>(
    visitor: &mut impl Visitor<'a>,
    variable: &'a ast::ASTVariable<'a>,
) {
    visitor.visit_any_variable(variable);
}

pub fn walk_const_variable<'a>(visitor: &mut impl Visitor<'a>, variable: &'a ast::ASTVariable<'a>) {
    visitor.visit_any_variable(variable);
}

pub fn walk_static_variable<'a>(
    visitor: &mut impl Visitor<'a>,
    variable: &'a ast::ASTVariable<'a>,
) {
    visitor.visit_any_variable(variable);
}

pub fn walk_binding_variable<'a>(
    visitor: &mut impl Visitor<'a>,
    variable: &'a ast::ASTVariable<'a>,
) {
    visitor.visit_any_variable(variable);
}

pub fn walk_any_variable<'a>(visitor: &mut impl Visitor<'a>, variable: &'a ast::ASTVariable<'a>) {
    visitor.visit_expr(&variable.identifier);

    if let Some(expr) = &variable.typehint {
        visitor.visit_expr(expr);
    }

    if let Some(expr) = &variable.value {
        visitor.visit_expr(expr);
    }
}

pub fn walk_exprs<'a>(visitor: &mut impl Visitor<'a>, exprs: &'a [ast::ASTExpr<'a>]) {
    for expr in exprs {
        visitor.visit_expr(expr);
    }
}

#[rustfmt::skip]
pub fn walk_expr<'a>(visitor: &mut impl Visitor<'a>, expr: &'a ast::ASTExpr<'a>) {
    match expr {
        ast::ASTExpr { span, kind: ast::ASTExprKind::Group(exprs) } => visitor.visit_group_expr(exprs, span),
        ast::ASTExpr { span, kind: ast::ASTExprKind::Identifier(identifier) } => visitor.visit_identifier_expr(identifier, span),
        ast::ASTExpr { span, kind: ast::ASTExprKind::Number(number) } => visitor.visit_number_expr(*number, span),
        ast::ASTExpr { span, kind: ast::ASTExprKind::Float(float) } => visitor.visit_float_expr(*float, span),
        ast::ASTExpr { span, kind: ast::ASTExprKind::String(string) } => visitor.visit_string_expr(string, span),
        ast::ASTExpr { span, kind: ast::ASTExprKind::StringName(string) } => visitor.visit_string_name_expr(string, span),
        ast::ASTExpr { span, kind: ast::ASTExprKind::Node(path) } => visitor.visit_node_expr(path, span),
        ast::ASTExpr { span, kind: ast::ASTExprKind::UniqueNode(path) } => visitor.visit_unique_node_expr(path, span),
        ast::ASTExpr { span, kind: ast::ASTExprKind::NodePath(path) } => visitor.visit_node_path_expr(path, span),
        ast::ASTExpr { span, kind: ast::ASTExprKind::Boolean(boolean) } => visitor.visit_boolean_expr(*boolean, span),
        ast::ASTExpr { span, kind: ast::ASTExprKind::Null } => visitor.visit_null_expr(span),
        ast::ASTExpr { span, kind: ast::ASTExprKind::Array(exprs) } => visitor.visit_array_expr(exprs, span),
        ast::ASTExpr { span, kind: ast::ASTExprKind::Dictionary(pairs) } => visitor.visit_dictionary_expr(pairs.as_slice(), span),
        ast::ASTExpr { span: _, kind: ast::ASTExprKind::Lambda(func) } => visitor.visit_lambda_expr(func),
        ast::ASTExpr { span: _, kind: ast::ASTExprKind::PrefixExpr(op, expr) } => visitor.visit_prefix_expr(op, expr),
        ast::ASTExpr { span, kind: ast::ASTExprKind::PostfixExpr(expr, op) } => visitor.visit_postfix_expr(expr, op, span),
        ast::ASTExpr { span, kind: ast::ASTExprKind::BinaryExpr(lhs, op, rhs) } => visitor.visit_binary_expr(lhs, op, rhs, span),
    }
}

pub fn walk_group_expr<'a>(
    visitor: &mut impl Visitor<'a>,
    exprs: &'a [ast::ASTExpr],
    _span: &'a Span,
) {
    visitor.visit_exprs(exprs);
}

pub fn walk_identifier_expr<'a>(
    _visitor: &mut impl Visitor<'a>,
    _identifier: &'a str,
    _span: &'a Span,
) {
}
pub fn walk_number_expr<'a>(_visitor: &mut impl Visitor<'a>, _number: u64, _span: &'a Span) {}
pub fn walk_float_expr<'a>(_visitor: &mut impl Visitor<'a>, _float: f64, _span: &'a Span) {}
pub fn walk_string_expr<'a>(_visitor: &mut impl Visitor<'a>, _string: &'a str, _span: &'a Span) {}
pub fn walk_string_name_expr<'a>(
    _visitor: &mut impl Visitor<'a>,
    _string: &'a str,
    _span: &'a Span,
) {
}
pub fn walk_node_expr<'a>(_visitor: &mut impl Visitor<'a>, _path: &'a str, _span: &'a Span) {}
pub fn walk_unique_node_expr<'a>(
    _visitor: &mut impl Visitor<'a>,
    _path: &'a str,
    _span: &'a Span,
) {
}
pub fn walk_node_path_expr<'a>(_visitor: &mut impl Visitor<'a>, _path: &'a str, _span: &'a Span) {
}
pub fn walk_boolean_expr<'a>(_visitor: &mut impl Visitor<'a>, _boolean: bool, _span: &'a Span) {}
pub fn walk_null_expr<'a>(_visitor: &mut impl Visitor<'a>, _span: &'a Span) {}

pub fn walk_array_expr<'a>(
    visitor: &mut impl Visitor<'a>,
    exprs: &'a [ast::ASTExpr],
    _span: &'a Span,
) {
    visitor.visit_exprs(exprs);
}

pub fn walk_dictionary_expr<'a>(
    visitor: &mut impl Visitor<'a>,
    pairs: &'a [ast::DictValue],
    _span: &'a Span,
) {
    for (key, value) in pairs {
        visitor.visit_expr(key);
        visitor.visit_expr(value)
    }
}

pub fn walk_lambda_expr<'a>(visitor: &mut impl Visitor<'a>, func: &'a ast::ASTFunction<'a>) {
    visitor.visit_func(func);
}

pub fn walk_prefix_expr<'a>(
    visitor: &mut impl Visitor<'a>,
    op: &'a ast::ASTPrefixOp,
    expr: &'a ast::ASTExpr<'a>,
) {
    // future updates can introduce new prefix ops that have associated data,
    // we we avoid `_ => ()` here
    match &op.kind {
        ast::ASTPrefixOpKind::Await => (),
        ast::ASTPrefixOpKind::Identity => (),
        ast::ASTPrefixOpKind::Negation => (),
        ast::ASTPrefixOpKind::Not => (),
        ast::ASTPrefixOpKind::BitwiseNot => (),
    }

    visitor.visit_expr(expr);
}

pub fn walk_postfix_expr<'a>(
    visitor: &mut impl Visitor<'a>,
    expr: &'a ast::ASTExpr,
    op: &'a ast::ASTPostfixOp,
    _span: &'a Span,
) {
    visitor.visit_expr(expr);

    match &op.kind {
        ast::ASTPostfixOpKind::Call(args) => visitor.visit_exprs(args.as_slice()),
        ast::ASTPostfixOpKind::Subscript(args) => visitor.visit_exprs(args.as_slice()),
    }
}

pub fn walk_binary_expr<'a>(
    visitor: &mut impl Visitor<'a>,
    lhs: &'a ast::ASTExpr,
    op: &'a ast::ASTBinaryOp,
    rhs: &'a ast::ASTExpr,
    _span: &'a Span,
) {
    visitor.visit_expr(lhs);

    // future updates can introduce new binary ops that have associated data,
    // we we avoid `_ => ()` here
    match &op {
        ast::ASTBinaryOp::LessThan => (),
        ast::ASTBinaryOp::LessOrEqual => (),
        ast::ASTBinaryOp::Greater => (),
        ast::ASTBinaryOp::GreaterOrEqual => (),
        ast::ASTBinaryOp::Equals => (),
        ast::ASTBinaryOp::NotEqual => (),
        ast::ASTBinaryOp::And => (),
        ast::ASTBinaryOp::Or => (),
        ast::ASTBinaryOp::BitwiseAnd => (),
        ast::ASTBinaryOp::BitwiseOr => (),
        ast::ASTBinaryOp::BitwiseXor => (),
        ast::ASTBinaryOp::BitwiseShiftLeft => (),
        ast::ASTBinaryOp::BitwiseShiftRight => (),
        ast::ASTBinaryOp::Add => (),
        ast::ASTBinaryOp::Subtract => (),
        ast::ASTBinaryOp::Multiply => (),
        ast::ASTBinaryOp::Power => (),
        ast::ASTBinaryOp::Divide => (),
        ast::ASTBinaryOp::Remainder => (),
        ast::ASTBinaryOp::TypeCast => (),
        ast::ASTBinaryOp::TypeCheck => (),
        ast::ASTBinaryOp::Contains => (),
        ast::ASTBinaryOp::NotContains => (),
        ast::ASTBinaryOp::PropertyAccess => (),
        ast::ASTBinaryOp::Range => (),
        ast::ASTBinaryOp::TernaryIfElse(condition) => visitor.visit_expr(condition),
        ast::ASTBinaryOp::TernaryIfElsePlaceholder => (),
        ast::ASTBinaryOp::Assignment => (),
        ast::ASTBinaryOp::PlusAssignment => (),
        ast::ASTBinaryOp::MinusAssignment => (),
        ast::ASTBinaryOp::MultiplyAssignment => (),
        ast::ASTBinaryOp::PowerAssignment => (),
        ast::ASTBinaryOp::DivideAssignment => (),
        ast::ASTBinaryOp::RemainderAssignment => (),
        ast::ASTBinaryOp::BitwiseAndAssignment => (),
        ast::ASTBinaryOp::BitwiseOrAssignment => (),
        ast::ASTBinaryOp::BitwiseNotAssignment => (),
        ast::ASTBinaryOp::BitwiseXorAssignment => (),
        ast::ASTBinaryOp::BitwiseShiftLeftAssignment => (),
        ast::ASTBinaryOp::BitwiseShiftRightAssignment => (),
    }

    visitor.visit_expr(rhs);
}
