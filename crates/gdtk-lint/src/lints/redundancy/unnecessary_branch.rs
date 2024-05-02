use diagnosis::{Diagnostic, Highlight, Severity};
use gdtk_ast::{ast, Visitor, visitor::walk_block};

#[derive(Default, Debug)]
struct BranchGroup<'a> {
    if_: Option<&'a ast::ASTIfStmt<'a>>,
    elifs: Vec<&'a ast::ASTElifStmt<'a>>,
    else_: Option<&'a ast::ASTElseStmt<'a>>,
}

impl BranchGroup<'_> {
    fn is_empty(&self) -> bool {
        self.if_.is_none() && self.elifs.is_empty() && self.else_.is_none()
    }
}

#[derive(Default, Debug)]
struct BranchGroups<'a> {
    inner: Vec<BranchGroup<'a>>,
}

impl<'a> BranchGroups<'a> {
    fn create(&mut self) {
        self.inner.push(BranchGroup::default());
    }

    fn last(&mut self) -> &mut BranchGroup<'a> {
        if self.inner.len() == 0 {
            self.create();
        }

        self.inner.last_mut().unwrap()
    }

    fn add_if(&mut self, stmt: &'a ast::ASTIfStmt<'a>) {
        if !self.last().is_empty() {
            self.create();
        }

        self.last().if_ = Some(stmt);
    }

    fn add_elif(&mut self, stmt: &'a ast::ASTElifStmt<'a>) {
        self.last().elifs.push(stmt);
    }

    fn add_else(&mut self, stmt: &'a ast::ASTElseStmt<'a>) {
        self.last().else_ = Some(stmt);
        self.create();
    }
}

impl<'a> IntoIterator for BranchGroups<'a> {
    type Item = <Vec<BranchGroup<'a>> as IntoIterator>::Item;

    type IntoIter = <Vec<BranchGroup<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> From<&'a [ast::ASTStatement<'a>]> for BranchGroups<'a> {
    fn from(block: &'a [ast::ASTStatement<'a>]) -> Self {
        let mut groups = Self::default();

        for stmt in block {
            match stmt {
                ast::ASTStatement::If(stmt) => groups.add_if(stmt),
                ast::ASTStatement::Elif(stmt) => groups.add_elif(stmt),
                ast::ASTStatement::Else(stmt) => groups.add_else(stmt),
                _ => (),
            }
        }

        groups
    }
}

crate::lint!(UnnecessaryBranch);

impl<'s> Visitor<'s> for UnnecessaryBranch<'s> {
    fn visit_block(&mut self, block: &'s [ast::ASTStatement<'s>]) {
        walk_block(self, block);

        let groups = BranchGroups::from(block);

        for BranchGroup { if_, elifs, else_ } in groups {
            if let Some(else_) = else_
                && (if_.is_none() || always_returns(if_.unwrap().block.as_slice()))
                && elifs
                    .into_iter()
                    .all(|elif| always_returns(elif.block.as_slice()))
            {
                self.0.push(
                    Diagnostic::new("Unnecessary `else`.", Severity::Warning)
                        .with_code("unnecessary-branch"), // .with_span(else_.span)
                                                          // .add_highlight(Highlight::new(else_.span))
                )
            }
        }
    }
}

fn always_returns(block: &[ast::ASTStatement]) -> bool {
    // A block always returns only if either:
    // - it has an unconditional `return`
    // - it has an `if/elif/else` chain all blocks of which always return

    if block.iter().any(|stmt| stmt.is_return()) {
        return true;
    }

    BranchGroups::from(block)
        .into_iter()
        .any(|group| {
            (group.if_.is_none() || always_returns(group.if_.unwrap().block.as_slice()))
            && (group.elifs.is_empty() || group.elifs.into_iter().all(|elif| always_returns(elif.block.as_slice()))
            && (group.else_.is_none() || always_returns(group.else_.unwrap().block.as_slice()))
        })
}
