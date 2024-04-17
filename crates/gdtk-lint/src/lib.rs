pub mod lints;

use gdtk_ast::Visitor;

pub fn run_builtin_lints(file: &gdtk_ast::ASTFile) -> Vec<Box<dyn miette::Diagnostic>> {
    let mut diagnostics: Vec<Box<dyn miette::Diagnostic>> = vec![];

    // Construct lints.
    let mut identifier_casing = crate::lints::style::identifier_casing::IdentifierCasing::new();

    // Run lints.
    identifier_casing.visit_file(file);

    // Collect diagnostics.
    diagnostics.extend(identifier_casing.diagnostics);

    diagnostics
}
