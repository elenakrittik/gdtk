pub mod gdscript;
pub mod godotcfg;

use crate::cli::dev::{gdscript::DevGDScriptCommand, godotcfg::DevGodotCfgCommand};

pub enum DevCommand {
    /// GDScript-related dev commands.
    GDScript(DevGDScriptCommand),
    /// GodotCfg-related dev commands.
    GodotCfg(DevGodotCfgCommand),
}

impl tapcli::Command for DevCommand {
    type Error = anyhow::Error;

    async fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        todo!()
    }

    async fn run(self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
