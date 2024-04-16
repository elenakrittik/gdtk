use gdtk_ast::poor::{ASTStatement, ASTValue};
use trustfall::provider::TrustfallEnumVertex;

#[derive(Debug, Clone, TrustfallEnumVertex)]
pub enum Vertex<'a> {
    Statement(&'a ASTStatement<'a>),
    #[allow(clippy::borrowed_box)]
    Value(&'a Box<ASTValue<'a>>),
}

impl Vertex<'_> {
    pub fn is_class_name_stmt(&self) -> bool {
        matches!(self, Vertex::Statement(ASTStatement::ClassName(_)))
    }

    pub fn is_value(&self) -> bool {
        matches!(self, Vertex::Value(_))
    }

    pub fn is_identifier_value(&self) -> bool {
        matches!(self, Vertex::Value(deref!(ASTValue::Identifier(_))))
    }

    pub fn is_binary_expr_value(&self) -> bool {
        matches!(self, Vertex::Value(deref!(ASTValue::BinaryExpr(_, _, _))))
    }
}
