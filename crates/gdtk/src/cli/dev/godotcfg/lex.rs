use std::path::PathBuf;

use crate::{cli::utils::ParserExt, utils::get_content};

pub struct DevGodotCfgLexCommand {
    /// The GodotCfg file to lex.
    file: PathBuf,
}

impl tapcli::Command for DevGodotCfgLexCommand {
    type Error = anyhow::Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let file = parser.next_value()?.into();

        Ok(Self { file })
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        let content = get_content(self.file.as_path())?;
        let lexed = gdtk_godotcfg_parser::lexer(&content);

        for token in lexed {
            eprintln!("{:?}", token);
        }

        Ok(())
    }
}
