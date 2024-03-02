use std::rc::Rc;

use gdtk_ast::poor::ASTFile;
use trustfall::provider::TrustfallEnumVertex;

#[derive(Debug, Clone, TrustfallEnumVertex)]
pub enum Vertex<'a> {
    File(Rc<ASTFile<'a>>),
}
