use gdtk_ast::{ast, Visitor};

crate::declare_lint!(
    SelfInStaticFunc,
    code = "gdtk::syntax::self_in_static_func",
    severity = Error
);

impl Visitor for SelfInStaticFunc {
    fn visit_func(&mut self, _func: &ast::ASTFunction) {
        todo!()
    }
}
