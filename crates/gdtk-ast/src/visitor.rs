use crate::ast;

pub trait Visitor {
    fn visit_file(&mut self, file: &ast::ASTFile) {
        self.visit_block(&file.body);
    }

    fn visit_block(&mut self, block: &[ast::ASTStatement]) {
        for stmt in block {
            self.visit_statement(stmt);
        }
    }

    #[rustfmt::skip]
    fn visit_statement(&mut self, stmt: &ast::ASTStatement) {
        match stmt {
            ast::ASTStatement::Annotation(ann) => self.visit_annotation(ann),
            ast::ASTStatement::Assert(expr) => self.visit_assert_statement(expr),
            ast::ASTStatement::Break => self.visit_break_statement(),
            ast::ASTStatement::Breakpoint => self.visit_breakpoint_statement(),
            ast::ASTStatement::Class(class) => self.visit_class(class),
            ast::ASTStatement::ClassName(identifier) => self.visit_class_name_statement(identifier),
            ast::ASTStatement::Continue => self.visit_continue_statement(),
            ast::ASTStatement::If(stmt) => self.visit_if_statement(stmt),
            ast::ASTStatement::Elif(stmt) => self.visit_elif_statement(stmt),
            ast::ASTStatement::Else(stmt) => self.visit_else_statement(stmt),
            ast::ASTStatement::Enum(stmt) => self.visit_enum_statement(stmt),
            ast::ASTStatement::Extends(identifier) => self.visit_extends_statement(identifier),
            ast::ASTStatement::For(stmt) => self.visit_for_statement(stmt),
            ast::ASTStatement::Func(func) => self.visit_func(func),
            ast::ASTStatement::Pass => self.visit_pass_statement(),
            ast::ASTStatement::Return(expr) => self.visit_return_statement(expr.as_ref()),
            ast::ASTStatement::Signal(stmt) => self.visit_signal_statement(stmt),
            ast::ASTStatement::Match(stmt) => self.visit_match_statement(stmt),
            ast::ASTStatement::While(stmt) => self.visit_while_statement(stmt),
            ast::ASTStatement::Variable(variable) => self.visit_variable(variable),
            ast::ASTStatement::Expr(expr) => self.visit_expr(expr),
        }
    }

    fn visit_annotation(&mut self, ann: &ast::ASTAnnotation) {
        self.visit_expr(&ast::ASTExpr::Identifier(ann.identifier));

        if let Some(args) = &ann.arguments {
            self.visit_exprs(args.as_slice());
        }
    }

    fn visit_assert_statement(&mut self, expr: &ast::ASTExpr) {
        self.visit_expr(expr);
    }

    fn visit_break_statement(&mut self) {}
    fn visit_breakpoint_statement(&mut self) {}

    fn visit_class(&mut self, class: &ast::ASTClass) {
        self.visit_expr(&ast::ASTExpr::Identifier(class.identifier));

        if let Some(extends) = class.extends {
            self.visit_expr(&ast::ASTExpr::Identifier(extends));
        }

        self.visit_block(class.body.as_slice());
    }

    fn visit_class_name_statement(&mut self, identifier: &str) {
        self.visit_expr(&ast::ASTExpr::Identifier(identifier));
    }

    fn visit_continue_statement(&mut self) {}

    fn visit_if_statement(&mut self, stmt: &ast::ASTIfStmt) {
        self.visit_expr(&stmt.expr);
        self.visit_block(stmt.block.as_slice());
    }

    fn visit_elif_statement(&mut self, stmt: &ast::ASTElifStmt) {
        self.visit_expr(&stmt.expr);
        self.visit_block(stmt.block.as_slice());
    }

    fn visit_else_statement(&mut self, stmt: &ast::ASTElseStmt) {
        self.visit_block(stmt.block.as_slice());
    }

    fn visit_enum_statement(&mut self, stmt: &ast::ASTEnum) {
        if let Some(identfier) = stmt.identifier {
            self.visit_expr(&ast::ASTExpr::Identifier(identfier));
        }

        self.visit_enum_variants(stmt.variants.as_slice());
    }

    fn visit_enum_variants(&mut self, variants: &[ast::ASTEnumVariant]) {
        for variant in variants {
            self.visit_enum_variant(variant);
        }
    }

    fn visit_enum_variant(&mut self, variant: &ast::ASTEnumVariant) {
        self.visit_expr(&ast::ASTExpr::Identifier(variant.identifier));

        if let Some(expr) = &variant.value {
            self.visit_expr(expr);
        }
    }

    fn visit_extends_statement(&mut self, identifier: &str) {
        self.visit_expr(&ast::ASTExpr::Identifier(identifier));
    }

    fn visit_for_statement(&mut self, stmt: &ast::ASTForStmt) {
        self.visit_variable(&stmt.binding);
        self.visit_expr(&stmt.container);
        self.visit_block(stmt.block.as_slice());
    }

    fn visit_func(&mut self, func: &ast::ASTFunction) {
        if let Some(identifier) = func.identifier {
            self.visit_expr(&ast::ASTExpr::Identifier(identifier));
        }

        self.visit_parameters(func.parameters.as_slice());

        if let Some(return_type) = &func.return_type {
            self.visit_expr(return_type);
        }

        self.visit_block(func.body.as_slice());
    }

    fn visit_parameters(&mut self, parameters: &[ast::ASTVariable]) {
        for param in parameters {
            self.visit_variable(param);
        }
    }

    fn visit_pass_statement(&mut self) {}

    fn visit_return_statement(&mut self, expr: Option<&ast::ASTExpr>) {
        if let Some(expr) = expr {
            self.visit_expr(expr);
        }
    }

    fn visit_signal_statement(&mut self, signal: &ast::ASTSignal) {
        self.visit_expr(&ast::ASTExpr::Identifier(signal.identifier));

        if let Some(params) = &signal.parameters {
            self.visit_parameters(params.as_slice());
        }
    }

    fn visit_match_statement(&mut self, stmt: &ast::ASTMatchStmt) {
        self.visit_expr(&stmt.expr);
        self.visit_match_arms(stmt.arms.as_slice());
    }

    fn visit_match_arms(&mut self, arms: &[ast::ASTMatchArm]) {
        for arm in arms {
            self.visit_match_arm(arm);
        }
    }

    fn visit_match_arm(&mut self, arm: &ast::ASTMatchArm) {
        self.visit_match_pattern(&arm.pattern);

        if let Some(guard) = &arm.guard {
            self.visit_match_guard(guard);
        }

        self.visit_block(arm.block.as_slice());
    }

    fn visit_match_patterns(&mut self, patterns: &[ast::ASTMatchPattern]) {
        for pattern in patterns {
            self.visit_match_pattern(pattern);
        }
    }

    #[rustfmt::skip]
    fn visit_match_pattern(&mut self, pattern: &ast::ASTMatchPattern) {
        match pattern {
            ast::ASTMatchPattern::Value(expr) => self.visit_expr(expr),
            ast::ASTMatchPattern::Binding(binding) => self.visit_match_binding_pattern(binding),
            ast::ASTMatchPattern::Array(subpatterns) => self.visit_match_array_pattern(subpatterns),
            ast::ASTMatchPattern::Dictionary(subpatterns) => self.visit_match_dictionary_pattern(subpatterns),
            ast::ASTMatchPattern::Alternative(subpatterns) => self.visit_match_alternative_pattern(subpatterns),
            ast::ASTMatchPattern::Ignore => self.visit_match_ignore_pattern(),
        }
    }

    fn visit_match_expr_pattern(&mut self, expr: &ast::ASTExpr) {
        self.visit_expr(expr);
    }

    fn visit_match_binding_pattern(&mut self, binding: &ast::ASTVariable) {
        self.visit_variable(binding);
    }

    fn visit_match_array_pattern(&mut self, subpatterns: &[ast::ASTMatchPattern]) {
        self.visit_match_patterns(subpatterns);
    }

    fn visit_match_dictionary_pattern(&mut self, subpatterns: &[ast::DictPattern]) {
        for (key, value) in subpatterns {
            self.visit_expr(key);

            if let Some(value) = value {
                self.visit_match_pattern(value);
            }
        }
    }

    fn visit_match_alternative_pattern(&mut self, subpatterns: &[ast::ASTMatchPattern]) {
        self.visit_match_patterns(subpatterns);
    }

    fn visit_match_ignore_pattern(&mut self) {}

    fn visit_match_guard(&mut self, expr: &ast::ASTExpr) {
        self.visit_expr(expr);
    }

    fn visit_while_statement(&mut self, stmt: &ast::ASTWhileStmt) {
        self.visit_expr(&stmt.expr);
        self.visit_block(stmt.block.as_slice());
    }

    #[rustfmt::skip]
    fn visit_variable(&mut self, variable: &ast::ASTVariable) {
        match variable {
            ast::ASTVariable { kind: ast::ASTVariableKind::Regular, .. } => self.visit_regular_variable(variable),
            ast::ASTVariable { kind: ast::ASTVariableKind::Constant, .. } => self.visit_const_variable(variable),
            ast::ASTVariable { kind: ast::ASTVariableKind::Static, .. } => self.visit_static_variable(variable),
            ast::ASTVariable { kind: ast::ASTVariableKind::Binding, .. } => self.visit_binding_variable(variable),
        }
    }

    fn visit_regular_variable(&mut self, variable: &ast::ASTVariable) {
        self.visit_any_variable(variable);
    }

    fn visit_const_variable(&mut self, variable: &ast::ASTVariable) {
        self.visit_any_variable(variable);
    }

    fn visit_static_variable(&mut self, variable: &ast::ASTVariable) {
        self.visit_any_variable(variable);
    }

    fn visit_binding_variable(&mut self, variable: &ast::ASTVariable) {
        self.visit_any_variable(variable);
    }

    fn visit_any_variable(&mut self, variable: &ast::ASTVariable) {
        self.visit_expr(&ast::ASTExpr::Identifier(variable.identifier));

        if let Some(expr) = &variable.typehint {
            self.visit_expr(expr);
        }

        if let Some(expr) = &variable.value {
            self.visit_expr(expr);
        }
    }

    fn visit_exprs(&mut self, exprs: &[ast::ASTExpr]) {
        for expr in exprs {
            self.visit_expr(expr);
        }
    }

    #[rustfmt::skip]
    fn visit_expr(&mut self, expr: &ast::ASTExpr) {
        match expr {
            ast::ASTExpr::Group(exprs) => self.visit_group_expr(exprs),
            ast::ASTExpr::Identifier(identifier) => self.visit_identifier_expr(identifier),
            ast::ASTExpr::Number(number) => self.visit_number_expr(*number),
            ast::ASTExpr::Float(float) => self.visit_float_expr(*float),
            ast::ASTExpr::String(string) => self.visit_string_expr(string),
            ast::ASTExpr::StringName(string) => self.visit_string_name_expr(string),
            ast::ASTExpr::Node(path) => self.visit_node_expr(path),
            ast::ASTExpr::UniqueNode(path) => self.visit_unique_node_expr(path),
            ast::ASTExpr::NodePath(path) => self.visit_node_path_expr(path),
            ast::ASTExpr::Boolean(boolean) => self.visit_boolean_expr(*boolean),
            ast::ASTExpr::Null => self.visit_null_expr(),
            ast::ASTExpr::Array(exprs) => self.visit_array_expr(exprs),
            ast::ASTExpr::Dictionary(pairs) => self.visit_dictionary_expr(pairs.as_slice()),
            ast::ASTExpr::Lambda(func) => self.visit_lambda_expr(func),
            ast::ASTExpr::PrefixExpr(op, expr) => self.visit_prefix_expr(op, expr),
            ast::ASTExpr::PostfixExpr(expr, op) => self.visit_postfix_expr(expr, op),
            ast::ASTExpr::BinaryExpr(lhs, op, rhs) => self.visit_binary_expr(lhs, op, rhs),
        }
    }

    fn visit_group_expr(&mut self, exprs: &[ast::ASTExpr]) {
        self.visit_exprs(exprs);
    }

    fn visit_identifier_expr(&mut self, _identifier: &str) {}
    fn visit_number_expr(&mut self, _number: u64) {}
    fn visit_float_expr(&mut self, _float: f64) {}
    fn visit_string_expr(&mut self, _string: &str) {}
    fn visit_string_name_expr(&mut self, _string: &str) {}
    fn visit_node_expr(&mut self, _path: &str) {}
    fn visit_unique_node_expr(&mut self, _path: &str) {}
    fn visit_node_path_expr(&mut self, _path: &str) {}
    fn visit_boolean_expr(&mut self, _boolean: bool) {}
    fn visit_null_expr(&mut self) {}

    fn visit_array_expr(&mut self, exprs: &[ast::ASTExpr]) {
        self.visit_exprs(exprs);
    }

    fn visit_dictionary_expr(&mut self, pairs: &[ast::DictValue]) {
        for (key, value) in pairs {
            self.visit_expr(key);
            self.visit_expr(value)
        }
    }

    fn visit_lambda_expr(&mut self, func: &ast::ASTFunction) {
        self.visit_func(func);
    }

    fn visit_prefix_expr(&mut self, _op: &ast::ASTPrefixOp, expr: &ast::ASTExpr) {
        self.visit_expr(expr);
    }

    fn visit_postfix_expr(&mut self, expr: &ast::ASTExpr, _op: &ast::ASTPostfixOp) {
        self.visit_expr(expr);
    }

    fn visit_binary_expr(&mut self, lhs: &ast::ASTExpr, _op: &ast::ASTBinaryOp, rhs: &ast::ASTExpr) {
        self.visit_expr(lhs);
        self.visit_expr(rhs);
    }
}
