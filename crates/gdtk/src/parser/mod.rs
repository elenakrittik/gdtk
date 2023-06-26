pub mod helpers;
pub mod meta;
pub mod statements;
pub mod values;

use crate::ast::ASTModule;
use crate::parser::{helpers::newlines, statements::statements};

pub fn parse(s: &mut String) -> anyhow::Result<ASTModule> {
    

    Ok(ASTModule { statements: vec![] })
}
