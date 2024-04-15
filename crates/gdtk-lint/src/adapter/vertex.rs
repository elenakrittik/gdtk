use gdtk_ast::poor::{ASTStatement, ASTValue};
use trustfall::provider::TrustfallEnumVertex;

#[derive(Debug, Clone, TrustfallEnumVertex)]
pub enum Vertex<'a> {
    Statement(&'a ASTStatement<'a>),
}

impl Vertex<'_> {
    pub fn is_class_name_stmt(&self) -> bool {
        matches!(self, Vertex::Statement(ASTStatement::ClassName(_)))
    }

    pub fn is_identifier_value(&self) -> bool {
        matches!(self, Vertex::Statement(ASTStatement::Value(ASTValue::Identifier(_))))
    }

    pub fn is_binary_expr_value(&self) -> bool {
        matches!(self, Vertex::Statement(ASTStatement::Value(ASTValue::BinaryExpr(_, _, _))))
    }
}
