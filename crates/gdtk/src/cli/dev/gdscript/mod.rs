use crate::cli::dev::gdscript::{lex::DevGDScriptLexCommand, parse::DevGDScriptParseCommand};

pub mod lex;
pub mod parse;

pub enum DevGDScriptCommand {
    /// Print the result of lexing the specified GDScript file.
    Lex(DevGDScriptLexCommand),
    /// Print the result of parsing the specified GDScript file.
    Parse(DevGDScriptParseCommand),
}

impl tapcli::Command for DevGDScriptCommand {
    type Error = anyhow::Error;

    async fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        todo!()
    }

    async fn run(self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
