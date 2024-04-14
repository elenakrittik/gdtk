use gdtk_ast::poor::ASTStatement;
use trustfall::provider::TrustfallEnumVertex;

#[derive(Debug, Clone, TrustfallEnumVertex)]
pub enum Vertex<'a> {
    Statement(&'a ASTStatement<'a>),
}
