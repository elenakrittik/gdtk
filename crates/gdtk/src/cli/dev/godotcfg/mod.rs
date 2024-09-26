use crate::cli::{
    dev::godotcfg::{lex::DevGodotCfgLexCommand, parse::DevGodotCfgParseCommand},
    unknown,
};

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

    #[rustfmt::skip]
    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        match parser.next().unwrap().as_ref() {
            tapcli::ArgRef::Value("lex") => Ok(Self::Lex(DevGodotCfgLexCommand::parse(parser)?)),
            tapcli::ArgRef::Value("parse") => Ok(Self::Parse(DevGodotCfgParseCommand::parse(parser)?)),
            other => unknown!(other),
        }
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        match self {
            Self::Lex(cmd) => cmd.run(),
            Self::Parse(cmd) => cmd.run(),
        }
    }
}
