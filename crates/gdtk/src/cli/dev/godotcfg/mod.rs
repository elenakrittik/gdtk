use crate::cli::dev::godotcfg::{lex::DevGodotCfgLexCommand, parse::DevGodotCfgParseCommand};

pub mod lex;
pub mod parse;

pub enum DevGodotCfgCommand {
    /// Print the result of lexing the specified GodotCfg file.
    Lex(DevGodotCfgLexCommand),
    /// Print the result of parsing the specified GodotCfg file.
    Parse(DevGodotCfgParseCommand),
}

impl tapcli::Command for DevGodotCfgCommand {
    type Error = anyhow::Error;

    async fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        todo!()
    }

    async fn run(self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
