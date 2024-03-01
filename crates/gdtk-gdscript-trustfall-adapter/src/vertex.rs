use gdtk_ast::poor::{ASTFile, ASTStatement};
use trustfall::provider::TrustfallEnumVertex;

#[derive(Debug, Clone, TrustfallEnumVertex)]
pub enum Vertex<'a> {
    File(&'a ASTFile<'a>),
    Statements(&'a Vec<ASTStatement<'a>>),
}
