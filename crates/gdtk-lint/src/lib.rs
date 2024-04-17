mod lints;

use gdtk_ast::Visitor;

pub fn run_builtin_lints(file: &gdtk_ast::ASTFile) {
    crate::lints::style::identifier_casing::IdentifierCasing.visit_file(file);
}
