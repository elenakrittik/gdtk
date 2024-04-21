#![feature(let_chains)]

pub mod lints;

use gdtk_ast::Visitor;

use crate::lints::{redundancy, style, syntax};

pub fn run_builtin_lints(file: &gdtk_ast::ASTFile) -> Vec<miette::MietteDiagnostic> {
    let mut diagnostics = vec![];

    // Construct lints.
    let mut identifier_casing = style::identifier_casing::IdentifierCasing::default();
    let mut unnecessary_pass = redundancy::unnecessary_pass::UnnecessaryPass::default();
    let mut invalid_assignment_target =
        syntax::invalid_assignment_target::InvalidAssignmentTarget::default();
    let mut self_in_static = syntax::self_in_static_func::SelfInStaticFunc::default();

    // Run lints.
    identifier_casing.visit_file(file);
    unnecessary_pass.visit_file(file);
    invalid_assignment_target.visit_file(file);
    self_in_static.visit_file(file);

    // Collect diagnostics.
    diagnostics.extend(identifier_casing.into_diagnostics());
    diagnostics.extend(unnecessary_pass.into_diagnostics());
    diagnostics.extend(invalid_assignment_target.into_diagnostics());
    diagnostics.extend(self_in_static.into_diagnostics());

    diagnostics
}
