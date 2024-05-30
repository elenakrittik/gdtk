use std::path::PathBuf;

use crate::utils::get_content;

pub struct DevGodotCfgParseCommand {
    /// The GodotCfg file to parse.
    file: PathBuf,
}

impl tapcli::Command for DevGodotCfgParseCommand {
    type Error = anyhow::Error;

    async fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        todo!()
    }

    async fn run(self) -> Result<Self::Output, Self::Error> {
        let content = get_content(self.file.as_path())?;
        let parsed = gdtk_godotcfg_parser::parser(&content);

        for line in parsed {
            println!("{:?}", line);
        }

        Ok(())
    }
}
