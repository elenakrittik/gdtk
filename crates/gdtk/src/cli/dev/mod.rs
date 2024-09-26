pub mod gdscript;
pub mod godotcfg;

use crate::cli::{
    dev::{gdscript::DevGDScriptCommand, godotcfg::DevGodotCfgCommand},
    unknown,
};

pub enum DevCommand {
    /// GDScript-related dev commands.
    GDScript(DevGDScriptCommand),
    /// GodotCfg-related dev commands.
    GodotCfg(DevGodotCfgCommand),
}

impl tapcli::Command for DevCommand {
    type Error = anyhow::Error;

    #[rustfmt::skip]
    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        match parser.next().unwrap().as_ref() {
            tapcli::ArgRef::Value("gdscript") => Ok(Self::GDScript(DevGDScriptCommand::parse(parser)?)),
            tapcli::ArgRef::Value("godotcfg") => Ok(Self::GodotCfg(DevGodotCfgCommand::parse(parser)?)),
            other => unknown!(other),
        }
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        match self {
            DevCommand::GDScript(c) => c.run(),
            DevCommand::GodotCfg(c) => c.run(),
        }
    }
}
