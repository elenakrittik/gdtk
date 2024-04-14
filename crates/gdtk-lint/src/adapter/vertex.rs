use gdtk_ast::poor::ASTStatement;
use trustfall::provider::TrustfallEnumVertex;

#[derive(Debug, Clone, TrustfallEnumVertex)]
pub enum Vertex<'a> {
    Statement(&'a ASTStatement<'a>),
    ClassNameStmt(&'a ASTStatement<'a>),
}

impl Vertex<'_> {
    pub fn is_class_name_stmt(&self) -> bool {
        matches!(self, Vertex::ClassNameStmt(_) | Vertex::Statement(ASTStatement::ClassName(_)))
    }
}
