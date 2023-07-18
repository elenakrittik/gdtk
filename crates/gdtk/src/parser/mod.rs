pub mod helpers;
pub mod meta;
pub mod statements;
pub mod values;

use self::statements::statement;
use crate::ast::ASTModule;

pub fn parse(input: &String) -> anyhow::Result<ASTModule> {
    let mut stmts = vec![];

    for line in input.split('\n') {
        stmts.push(statement(line)?);
    }

    Ok(ASTModule { statements: stmts })
}
