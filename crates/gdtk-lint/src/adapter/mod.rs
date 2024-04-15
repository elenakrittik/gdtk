mod vertex;

use gdtk_ast::poor::ASTFile;
use trustfall::provider::{resolve_property_with, BasicAdapter};

use crate::adapter::vertex::Vertex;

pub struct GDScriptAdapter<'a> {
    file: &'a ASTFile<'a>,
}

impl<'a> GDScriptAdapter<'a> {
    pub fn new(file: &'a ASTFile<'a>) -> Self {
        Self { file }
    }
}

impl<'a> BasicAdapter<'a> for GDScriptAdapter<'a> {
    type Vertex = Vertex<'a>;

    fn resolve_starting_vertices(
        &self,
        edge_name: &str,
        _parameters: &trustfall::provider::EdgeParameters,
    ) -> trustfall::provider::VertexIterator<'a, Self::Vertex> {
        match edge_name {
            "Statement" => Box::new(self.file.body.iter().map(Vertex::Statement)),
            _ => unreachable!(),
        }
    }

    fn resolve_property<V: trustfall::provider::AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: trustfall::provider::ContextIterator<'a, V>,
        type_name: &str,
        property_name: &str,
    ) -> trustfall::provider::ContextOutcomeIterator<'a, V, trustfall::FieldValue> {
        match (type_name, property_name) {
            ("ClassNameStmt", "identifier") => {
                let resolver =
                    |v: &Self::Vertex| v.as_statement().unwrap().as_class_name().copied().into();
                resolve_property_with(contexts, resolver)
            },
            ("IdentifierValue", "inner") => {
                let resolver = |v: &Self::Vertex| v.as_statement().unwrap().as_value().unwrap().as_identifier().copied().into();
                resolve_property_with(contexts, resolver)
            },
            ("BinaryExprValue", field) => {
                let idx = match field {
                    "left" => 0,
                    "op" => 1,
                    "right" => 2,
                    _ => unreachable!(),
                };

                let resolver = |v: &Self::Vertex| v.as_statement().unwrap().as_value().unwrap().as_binary_expr().map(|tuple_| tuple[idx]).into();
                resolve_property_with(contexts, resolver)
            }
            _ => unreachable!(),
        }
    }

    fn resolve_neighbors<V: trustfall::provider::AsVertex<Self::Vertex> + 'a>(
        &self,
        _contexts: trustfall::provider::ContextIterator<'a, V>,
        _type_name: &str,
        _edge_name: &str,
        _parameters: &trustfall::provider::EdgeParameters,
    ) -> trustfall::provider::ContextOutcomeIterator<
        'a,
        V,
        trustfall::provider::VertexIterator<'a, Self::Vertex>,
    > {
        todo!()
    }

    fn resolve_coercion<V: trustfall::provider::AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: trustfall::provider::ContextIterator<'a, V>,
        type_name: &str,
        coerce_to_type: &str,
    ) -> trustfall::provider::ContextOutcomeIterator<'a, V, bool> {
        let (type_name, coerce_to_type) = (type_name.to_owned(), coerce_to_type.to_owned());

        let iterator = contexts.map(move |ctx| {
            let vertex = match ctx.active_vertex() {
                Some(t) => t,
                None => return (ctx, false),
            };

            // Possible optimization here:
            // This "match" is loop-invariant, and can be hoisted outside the map() call
            // at the cost of a bit of code repetition.

            let can_coerce = match (type_name.as_ref(), coerce_to_type.as_ref()) {
                ("Statement", "ClassNameStmt") => vertex.is_class_name_stmt(),
                ("Statement", "IdentifierValue") => vertex.is_identifier_value(),
                ("Statement", "BinaryExprValue") => vertex.is_binary_expr_value(),
                unhandled => unreachable!("{:?}", unhandled),
            };

            (ctx, can_coerce)
        });

        Box::new(iterator)
    }
}
