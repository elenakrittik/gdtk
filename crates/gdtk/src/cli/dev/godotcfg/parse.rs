use std::path::PathBuf;

use crate::{cli::utils::ParserExt, utils::get_content};

pub struct DevGodotCfgParseCommand {
    /// The GodotCfg file to parse.
    file: PathBuf,
}

impl tapcli::Command for DevGodotCfgParseCommand {
    type Error = anyhow::Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let file = parser.next_value()?.into();

        Ok(Self { file })
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        let content = get_content(self.file.as_path())?;
        let parsed = gdtk_godotcfg_parser::parser(&content);

        for line in parsed {
            eprintln!("{:?}", line);
        }

        Ok(())
    }
}
