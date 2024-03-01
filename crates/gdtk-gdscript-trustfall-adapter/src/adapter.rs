use std::rc::Rc;

use gdtk_ast::poor::ASTFile;
use trustfall::provider::BasicAdapter;

use crate::vertex::Vertex;

pub struct GDScriptAdapter<'a> {
    file: Rc<ASTFile<'a>>,
}

impl<'a> GDScriptAdapter<'a> {
    pub fn new(file: ASTFile<'a>) -> Self {
        Self { file: Rc::new(file) }
    }
}

impl<'a> BasicAdapter<'a> for GDScriptAdapter<'a> {
    type Vertex = Vertex<'a>;

    fn resolve_starting_vertices(
        &self,
        edge_name: &str,
        parameters: &trustfall::provider::EdgeParameters,
    ) -> trustfall::provider::VertexIterator<'a, Self::Vertex> {
        match edge_name {
            "File" => {
                todo!()
            }
            "Statement" => {
                todo!()
            }
            _ => todo!()
        }
    }

    fn resolve_property<V: trustfall::provider::AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: trustfall::provider::ContextIterator<'a, V>,
        type_name: &str,
        property_name: &str,
    ) -> trustfall::provider::ContextOutcomeIterator<'a, V, trustfall::FieldValue> {
        todo!()
    }

    fn resolve_neighbors<V: trustfall::provider::AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: trustfall::provider::ContextIterator<'a, V>,
        type_name: &str,
        edge_name: &str,
        parameters: &trustfall::provider::EdgeParameters,
    ) -> trustfall::provider::ContextOutcomeIterator<'a, V, trustfall::provider::VertexIterator<'a, Self::Vertex>> {
        todo!()
    }

    fn resolve_coercion<V: trustfall::provider::AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: trustfall::provider::ContextIterator<'a, V>,
        type_name: &str,
        coerce_to_type: &str,
    ) -> trustfall::provider::ContextOutcomeIterator<'a, V, bool> {
        todo!()
    }
}
