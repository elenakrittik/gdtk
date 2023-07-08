pub mod helpers;
pub mod meta;
pub mod statements;
pub mod values;

use self::statements::statement;
use crate::ast::ASTModule;

pub fn parse(s: &mut String) -> anyhow::Result<ASTModule> {
    sparsec::from_string!(parser, s);

    let mut stmts = vec![];

    while let Ok(line) = parser.read_until("\n") {
        if line.is_empty() {
            continue;
        };

        stmts.push(statement(line)?);
    }

    Ok(ASTModule { statements: stmts })
}
