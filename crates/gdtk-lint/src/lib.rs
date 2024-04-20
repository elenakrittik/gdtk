#![feature(let_chains)]

pub mod lints;
pub mod utils;

use gdtk_ast::Visitor;

pub fn run_builtin_lints(file: &gdtk_ast::ASTFile) -> Vec<miette::MietteDiagnostic> {
    let mut diagnostics = vec![];

    // Construct lints.
    let mut identifier_casing = crate::lints::style::identifier_casing::IdentifierCasing(vec![]);
    let mut unnecessary_pass = crate::lints::redundancy::unnecessary_pass::UnnecessaryPass(vec![]);
    let mut invalid_assignment_target = crate::lints::syntax::invalid_assignment_target::InvalidAssignmentTarget(vec![]);

    // Run lints.
    identifier_casing.visit_file(file);
    unnecessary_pass.visit_file(file);
    invalid_assignment_target.visit_file(file);

    // Collect diagnostics.
    diagnostics.extend(identifier_casing.0);
    diagnostics.extend(unnecessary_pass.0);
    diagnostics.extend(invalid_assignment_target.0);

    diagnostics
}
