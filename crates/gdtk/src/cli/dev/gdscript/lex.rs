use std::path::PathBuf;

use itertools::Itertools;

use crate::{cli::utils::ParserExt, utils::get_content};

pub struct DevGDScriptLexCommand {
    /// The GDScript file to lex.
    file: PathBuf,
}

impl tapcli::Command for DevGDScriptLexCommand {
    type Error = anyhow::Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let file = parser.next_value()?.into();

        Ok(Self { file })
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        let content = get_content(self.file.as_path())?;
        let lexed = gdtk_gdscript_parser::lexer::lex(&content);

        eprintln!("Lexer output:\n```ron\n{:#?}\n```", &lexed.collect_vec());

        Ok(())
    }
}
