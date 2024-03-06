use gdtk_ast::poor::ASTFile;
use trustfall::provider::{resolve_property_with, BasicAdapter};

use crate::vertex::Vertex;

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
            "ClassName" => Box::new(
                self.file.body.iter()
                .filter_map(|s| s.as_class_name().map(|c| Vertex::ClassName(*c)))
            ),
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
            ("ClassName", "identifier") => {
                let resolver = |v: &Vertex| (*v.as_class_name().unwrap()).into();
                resolve_property_with(contexts, resolver)
            },
            _ => unreachable!(),
        }
    }

    fn resolve_neighbors<V: trustfall::provider::AsVertex<Self::Vertex> + 'a>(
        &self,
        _contexts: trustfall::provider::ContextIterator<'a, V>,
        _type_name: &str,
        _edge_name: &str,
        _parameters: &trustfall::provider::EdgeParameters,
    ) -> trustfall::provider::ContextOutcomeIterator<'a, V, trustfall::provider::VertexIterator<'a, Self::Vertex>> {
        todo!()
    }

    fn resolve_coercion<V: trustfall::provider::AsVertex<Self::Vertex> + 'a>(
        &self,
        _contexts: trustfall::provider::ContextIterator<'a, V>,
        _type_name: &str,
        _coerce_to_type: &str,
    ) -> trustfall::provider::ContextOutcomeIterator<'a, V, bool> {
        todo!()
    }
}
