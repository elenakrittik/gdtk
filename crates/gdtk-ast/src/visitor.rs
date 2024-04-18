use crate::ast;

type Range = std::ops::Range<usize>;

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
            ast::ASTStatement { range, kind: ast::ASTStatementKind::Annotation(ann) } => self.visit_annotation(ann, range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::Assert(expr) } => self.visit_assert_statement(expr, range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::Break } => self.visit_break_statement(range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::Breakpoint } => self.visit_breakpoint_statement(range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::Class(class) } => self.visit_class(class, range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::ClassName(identifier) } => self.visit_class_name_statement(identifier, range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::Continue } => self.visit_continue_statement(range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::If(stmt) } => self.visit_if_statement(stmt, range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::Elif(stmt) } => self.visit_elif_statement(stmt, range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::Else(stmt) } => self.visit_else_statement(stmt, range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::Enum(stmt) } => self.visit_enum_statement(stmt, range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::Extends(identifier) } => self.visit_extends_statement(identifier, range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::For(stmt) } => self.visit_for_statement(stmt, range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::Func(func) } => self.visit_func(func, range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::Pass } => self.visit_pass_statement(range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::Return(expr) } => self.visit_return_statement(expr.as_ref(), range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::Signal(stmt) } => self.visit_signal_statement(stmt, range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::Match(stmt) } => self.visit_match_statement(stmt, range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::While(stmt) } => self.visit_while_statement(stmt, range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::Variable(variable) } => self.visit_variable(variable, range),
            ast::ASTStatement { range, kind: ast::ASTStatementKind::Expr(expr) } => self.visit_expr(expr, range),
        }
    }

    fn visit_annotation(&mut self, ann: &ast::ASTAnnotation, range: Range) {
        self.visit_expr(&ann.identifier);

        if let Some(args) = &ann.arguments {
            self.visit_exprs(args.as_slice());
        }
    }

    fn visit_assert_statement(&mut self, expr: &ast::ASTExpr, range: Range) {
        self.visit_expr(expr);
    }

    fn visit_break_statement(&mut self, range: Range) {}
    fn visit_breakpoint_statement(&mut self, range: Range) {}

    fn visit_class(&mut self, class: &ast::ASTClass, range: Range) {
        self.visit_expr(&class.identifier);

        if let Some(extends) = &class.extends {
            self.visit_expr(extends);
        }

        self.visit_block(class.body.as_slice());
    }

    fn visit_class_name_statement(&mut self, identifier: &ast::ASTExpr, range: Range) {
        self.visit_expr(identifier);
    }

    fn visit_continue_statement(&mut self, range: Range) {}

    fn visit_if_statement(&mut self, stmt: &ast::ASTIfStmt, range: Range) {
        self.visit_expr(&stmt.expr);
        self.visit_block(stmt.block.as_slice());
    }

    fn visit_elif_statement(&mut self, stmt: &ast::ASTElifStmt, range: Range) {
        self.visit_expr(&stmt.expr);
        self.visit_block(stmt.block.as_slice());
    }

    fn visit_else_statement(&mut self, stmt: &ast::ASTElseStmt, range: Range) {
        self.visit_block(stmt.block.as_slice());
    }

    fn visit_enum_statement(&mut self, enum_: &ast::ASTEnum, range: Range) {
        if let Some(identfier) = &enum_.identifier {
            self.visit_expr(identfier)
        }

        self.visit_enum_variants(enum_.variants.as_slice());
    }

    fn visit_enum_variants(&mut self, variants: &[ast::ASTEnumVariant], range: Range) {
        for variant in variants {
            self.visit_enum_variant(variant);
        }
    }

    fn visit_enum_variant(&mut self, variant: &ast::ASTEnumVariant, range: Range) {
        self.visit_expr(&variant.identifier);

        if let Some(expr) = &variant.value {
            self.visit_expr(expr);
        }
    }

    fn visit_extends_statement(&mut self, identifier: &ast::ASTExpr, range: Range) {
        self.visit_expr(identifier)
    }

    fn visit_for_statement(&mut self, stmt: &ast::ASTForStmt, range: Range) {
        self.visit_variable(&stmt.binding);
        self.visit_expr(&stmt.container);
        self.visit_block(stmt.block.as_slice());
    }

    fn visit_func(&mut self, func: &ast::ASTFunction, range: Range) {
        if let Some(identifier) = &func.identifier {
            self.visit_expr(identifier)
        }

        self.visit_parameters(func.parameters.as_slice());

        if let Some(return_type) = &func.return_type {
            self.visit_expr(return_type);
        }

        self.visit_block(func.body.as_slice());
    }

    fn visit_parameters(&mut self, parameters: &[ast::ASTVariable], range: Range) {
        for param in parameters {
            self.visit_variable(param);
        }
    }

    fn visit_pass_statement(&mut self, range: Range) {}

    fn visit_return_statement(&mut self, expr: Option<&ast::ASTExpr>, range: Range) {
        if let Some(expr) = expr {
            self.visit_expr(expr);
        }
    }

    fn visit_signal_statement(&mut self, signal: &ast::ASTSignal, range: Range) {
        self.visit_expr(&signal.identifier);

        if let Some(params) = &signal.parameters {
            self.visit_parameters(params.as_slice());
        }
    }

    fn visit_match_statement(&mut self, stmt: &ast::ASTMatchStmt, range: Range) {
        self.visit_expr(&stmt.expr);
        self.visit_match_arms(stmt.arms.as_slice());
    }

    fn visit_match_arms(&mut self, arms: &[ast::ASTMatchArm], range: Range) {
        for arm in arms {
            self.visit_match_arm(arm);
        }
    }

    fn visit_match_arm(&mut self, arm: &ast::ASTMatchArm, range: Range) {
        self.visit_match_pattern(&arm.pattern);

        if let Some(guard) = &arm.guard {
            self.visit_match_guard(guard);
        }

        self.visit_block(arm.block.as_slice());
    }

    fn visit_match_patterns(&mut self, patterns: &[ast::ASTMatchPattern], range: Range) {
        for pattern in patterns {
            self.visit_match_pattern(pattern);
        }
    }

    #[rustfmt::skip]
    fn visit_match_pattern(&mut self, pattern: &ast::ASTMatchPattern, range: Range) {
        match pattern {
            ast::ASTMatchPattern::Value(expr) => self.visit_expr(expr),
            ast::ASTMatchPattern::Binding(binding) => self.visit_match_binding_pattern(binding),
            ast::ASTMatchPattern::Array(subpatterns) => self.visit_match_array_pattern(subpatterns),
            ast::ASTMatchPattern::Dictionary(subpatterns) => self.visit_match_dictionary_pattern(subpatterns),
            ast::ASTMatchPattern::Alternative(subpatterns) => self.visit_match_alternative_pattern(subpatterns),
            ast::ASTMatchPattern::Ignore => self.visit_match_ignore_pattern(),
        }
    }

    fn visit_match_expr_pattern(&mut self, expr: &ast::ASTExpr, range: Range) {
        self.visit_expr(expr);
    }

    fn visit_match_binding_pattern(&mut self, binding: &ast::ASTVariable, range: Range) {
        self.visit_variable(binding);
    }

    fn visit_match_array_pattern(&mut self, subpatterns: &[ast::ASTMatchPattern], range: Range) {
        self.visit_match_patterns(subpatterns);
    }

    fn visit_match_dictionary_pattern(&mut self, subpatterns: &[ast::DictPattern], range: Range) {
        for (key, value) in subpatterns {
            self.visit_expr(key);

            if let Some(value) = value {
                self.visit_match_pattern(value);
            }
        }
    }

    fn visit_match_alternative_pattern(
        &mut self,
        subpatterns: &[ast::ASTMatchPattern],
        range: Range,
    ) {
        self.visit_match_patterns(subpatterns);
    }

    fn visit_match_ignore_pattern(&mut self, range: Range) {}

    fn visit_match_guard(&mut self, expr: &ast::ASTExpr, range: Range) {
        self.visit_expr(expr);
    }

    fn visit_while_statement(&mut self, stmt: &ast::ASTWhileStmt, range: Range) {
        self.visit_expr(&stmt.expr);
        self.visit_block(stmt.block.as_slice());
    }

    #[rustfmt::skip]
    fn visit_variable(&mut self, variable: &ast::ASTVariable, range: Range) {
        match variable {
            ast::ASTVariable { kind: ast::ASTVariableKind::Regular, .. } => self.visit_regular_variable(variable),
            ast::ASTVariable { kind: ast::ASTVariableKind::Constant, .. } => self.visit_const_variable(variable),
            ast::ASTVariable { kind: ast::ASTVariableKind::Static, .. } => self.visit_static_variable(variable),
            ast::ASTVariable { kind: ast::ASTVariableKind::Binding, .. } => self.visit_binding_variable(variable),
        }
    }

    fn visit_regular_variable(&mut self, variable: &ast::ASTVariable, range: Range) {
        self.visit_any_variable(variable);
    }

    fn visit_const_variable(&mut self, variable: &ast::ASTVariable, range: Range) {
        self.visit_any_variable(variable);
    }

    fn visit_static_variable(&mut self, variable: &ast::ASTVariable, range: Range) {
        self.visit_any_variable(variable);
    }

    fn visit_binding_variable(&mut self, variable: &ast::ASTVariable, range: Range) {
        self.visit_any_variable(variable);
    }

    fn visit_any_variable(&mut self, variable: &ast::ASTVariable, range: Range) {
        self.visit_expr(&variable.identifier);

        if let Some(expr) = &variable.typehint {
            self.visit_expr(expr);
        }

        if let Some(expr) = &variable.value {
            self.visit_expr(expr);
        }
    }

    fn visit_exprs(&mut self, exprs: &[ast::ASTExpr], range: Range) {
        for expr in exprs {
            self.visit_expr(expr);
        }
    }

    #[rustfmt::skip]
    fn visit_expr(&mut self, expr: &ast::ASTExpr) {
        match expr {
            ast::ASTExpr { range, kind: ast::ASTExprKind::Group(exprs) } => self.visit_group_expr(exprs),
            ast::ASTExpr { range, kind: ast::ASTExprKind::Identifier(identifier) } => self.visit_identifier_expr(identifier),
            ast::ASTExpr { range, kind: ast::ASTExprKind::Number(number) } => self.visit_number_expr(*number),
            ast::ASTExpr { range, kind: ast::ASTExprKind::Float(float) } => self.visit_float_expr(*float),
            ast::ASTExpr { range, kind: ast::ASTExprKind::String(string) } => self.visit_string_expr(string),
            ast::ASTExpr { range, kind: ast::ASTExprKind::StringName(string) } => self.visit_string_name_expr(string),
            ast::ASTExpr { range, kind: ast::ASTExprKind::Node(path) } => self.visit_node_expr(path),
            ast::ASTExpr { range, kind: ast::ASTExprKind::UniqueNode(path) } => self.visit_unique_node_expr(path),
            ast::ASTExpr { range, kind: ast::ASTExprKind::NodePath(path) } => self.visit_node_path_expr(path),
            ast::ASTExpr { range, kind: ast::ASTExprKind::Boolean(boolean) } => self.visit_boolean_expr(*boolean),
            ast::ASTExpr { range, kind: ast::ASTExprKind::Null } => self.visit_null_expr(),
            ast::ASTExpr { range, kind: ast::ASTExprKind::Array(exprs) } => self.visit_array_expr(exprs),
            ast::ASTExpr { range, kind: ast::ASTExprKind::Dictionary(pairs) } => self.visit_dictionary_expr(pairs.as_slice()),
            ast::ASTExpr { range, kind: ast::ASTExprKind::Lambda(func) } => self.visit_lambda_expr(func),
            ast::ASTExpr { range, kind: ast::ASTExprKind::PrefixExpr(op, expr) } => self.visit_prefix_expr(op, expr),
            ast::ASTExpr { range, kind: ast::ASTExprKind::PostfixExpr(expr, op) } => self.visit_postfix_expr(expr, op),
            ast::ASTExpr { range, kind: ast::ASTExprKind::BinaryExpr(lhs, op, rhs) } => self.visit_binary_expr(lhs, op, rhs),
        }
    }

    fn visit_group_expr(&mut self, exprs: &[ast::ASTExpr], range: Range) {
        self.visit_exprs(exprs);
    }

    fn visit_identifier_expr(&mut self, _identifier: &str, range: Range) {}
    fn visit_number_expr(&mut self, _number: u64, range: Range) {}
    fn visit_float_expr(&mut self, _float: f64, range: Range) {}
    fn visit_string_expr(&mut self, _string: &str, range: Range) {}
    fn visit_string_name_expr(&mut self, _string: &str, range: Range) {}
    fn visit_node_expr(&mut self, _path: &str, range: Range) {}
    fn visit_unique_node_expr(&mut self, _path: &str, range: Range) {}
    fn visit_node_path_expr(&mut self, _path: &str, range: Range) {}
    fn visit_boolean_expr(&mut self, _boolean: bool, range: Range) {}
    fn visit_null_expr(&mut self, range: Range) {}

    fn visit_array_expr(&mut self, exprs: &[ast::ASTExpr], range: Range) {
        self.visit_exprs(exprs);
    }

    fn visit_dictionary_expr(&mut self, pairs: &[ast::DictValue], range: Range) {
        for (key, value) in pairs {
            self.visit_expr(key);
            self.visit_expr(value)
        }
    }

    fn visit_lambda_expr(&mut self, func: &ast::ASTFunction, range: Range) {
        self.visit_func(func);
    }

    fn visit_prefix_expr(&mut self, _op: &ast::ASTPrefixOp, expr: &ast::ASTExpr, range: Range) {
        self.visit_expr(expr);
    }

    fn visit_postfix_expr(&mut self, expr: &ast::ASTExpr, _op: &ast::ASTPostfixOp, range: Range) {
        self.visit_expr(expr);
    }

    fn visit_binary_expr(
        &mut self,
        lhs: &ast::ASTExpr,
        _op: &ast::ASTBinaryOp,
        rhs: &ast::ASTExpr,
        range: Range,
    ) {
        self.visit_expr(lhs);
        self.visit_expr(rhs);
    }
}
