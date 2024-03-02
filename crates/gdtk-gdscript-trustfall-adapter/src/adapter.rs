use std::rc::Rc;

use gdtk_ast::poor::ASTFile;
use trustfall::provider::{field_property, resolve_property_with, BasicAdapter};

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
        Box::new(match edge_name {
            "File" => {
                std::iter::once(Vertex::File(self.file.clone()))
            }
            "Statements" => {
                todo!()
            }
            _ => unimplemented!("unexpected starting edge: {edge_name}"),
        })
    }

    fn resolve_property<V: trustfall::provider::AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: trustfall::provider::ContextIterator<'a, V>,
        type_name: &str,
        property_name: &str,
    ) -> trustfall::provider::ContextOutcomeIterator<'a, V, trustfall::FieldValue> {
        match (type_name, property_name) {
            ("File", "body") => resolve_property_with(contexts, field_property!(as_file, body)),
            _ => unreachable!(),
        }
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
