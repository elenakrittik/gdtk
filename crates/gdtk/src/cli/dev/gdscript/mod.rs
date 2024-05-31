use crate::cli::{
    dev::gdscript::{lex::DevGDScriptLexCommand, parse::DevGDScriptParseCommand},
    unknown,
};

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

    #[rustfmt::skip]
    async fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        match parser.next().unwrap().as_ref() {
            tapcli::ArgRef::Value("lex") => Ok(Self::Lex(DevGDScriptLexCommand::parse(parser).await?)),
            tapcli::ArgRef::Value("parse") => Ok(Self::Parse(DevGDScriptParseCommand::parse(parser).await?)),
            other => unknown!(other),
        }
    }

    async fn run(self) -> Result<Self::Output, Self::Error> {
        match self {
            Self::Lex(cmd) => cmd.run().await,
            Self::Parse(cmd) => cmd.run().await,
        }
    }
}
