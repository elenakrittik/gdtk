#![feature(decl_macro)]

pub mod lints;
pub mod utils;

use gdtk_ast::Visitor;

pub fn run_builtin_lints(file: &gdtk_ast::ASTFile) -> Vec<miette::MietteDiagnostic> {
    let mut diagnostics = vec![];

    // Construct lints.
    let mut identifier_casing = crate::lints::style::identifier_casing::IdentifierCasing(vec![]);
    let mut unnecessary_pass = crate::lints::redundancy::unnecessary_pass::UnnecessaryPass(vec![]);

    // Run lints.
    identifier_casing.visit_file(file);
    unnecessary_pass.visit_file(file);

    // Collect diagnostics.
    diagnostics.extend(identifier_casing.0);
    diagnostics.extend(unnecessary_pass.0);

    diagnostics
}
