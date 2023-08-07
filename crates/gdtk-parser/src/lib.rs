use gdtk_ast::poor::ASTClass;
use gdtk_lexer::LexOutput;
use crate::error::Error;

pub mod error;
pub mod stage_0;

pub fn parse_tokens<'a>(tokens: LexOutput<'a>) -> Result<ASTClass, Error> {
    let tokens = crate::stage_0::run(tokens);

    todo!();
}
