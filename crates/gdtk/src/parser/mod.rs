pub mod helpers;
pub mod meta;
pub mod statements;
pub mod values;

use combine::{eof, Parser, Stream};

use crate::ast::ASTModule;
use crate::parser::{helpers::newlines, statements::statements};

pub fn parser<Input>() -> impl Parser<Input, Output = ASTModule>
where
    Input: Stream<Token = char>,
{
    (
        newlines::<Input>().silent(),
        statements(),
        newlines().silent(),
        eof(),
    )
        .map(|(_, statements, _, _)| ASTModule { statements })
}
