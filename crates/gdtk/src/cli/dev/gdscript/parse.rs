use std::path::PathBuf;

use crate::{cli::utils::ParserExt, utils::get_content};

pub struct DevGDScriptParseCommand {
    /// The GDScript file to parse.
    file: PathBuf,
}

impl tapcli::Command for DevGDScriptParseCommand {
    type Error = anyhow::Error;

    async fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let file = parser.next_value()?.into();

        Ok(Self { file })
    }

    async fn run(self) -> Result<Self::Output, Self::Error> {
        let content = get_content(self.file.as_path())?;
        let lexed = gdtk_gdscript_parser::lexer::lex(&content);
        let parsed = gdtk_gdscript_parser::parse_file(lexed);

        eprintln!("Parser output:\n```ron\n{:#?}\n```", &parsed);

        Ok(())
    }
}
