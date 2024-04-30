#![feature(let_chains, decl_macro)]

pub mod lints;
pub mod utils;

use gdtk_ast::Visitor;

use crate::lints::{redundancy, style, syntax};

pub fn run_builtin_lints<'s>(file: &'s gdtk_ast::ASTFile) -> Vec<diagnosis::Diagnostic<'s>> {
    let mut diagnostics = vec![];

    // Construct lints.
    let mut identifier_casing = style::identifier_case::IdentifierCase::default();
    let mut unnecessary_pass = redundancy::unnecessary_pass::UnnecessaryPass::default();
    let mut invalid_assignment_target =
        syntax::invalid_assignment_target::InvalidAssignmentTarget::default();
    let mut self_in_static = syntax::self_in_static_func::SelfInStaticFunc::default();
    let mut standalone_expression = redundancy::standalone_expression::StandaloneExpression::default();

    // Run lints.
    identifier_casing.visit_file(file);
    unnecessary_pass.visit_file(file);
    invalid_assignment_target.visit_file(file);
    self_in_static.visit_file(file);
    standalone_expression.visit_file(file);

    // Collect diagnostics.
    diagnostics.extend(identifier_casing.0);
    diagnostics.extend(unnecessary_pass.0);
    diagnostics.extend(invalid_assignment_target.0);
    diagnostics.extend(self_in_static.diagnostics);
    diagnostics.extend(standalone_expression.0);

    diagnostics
}
