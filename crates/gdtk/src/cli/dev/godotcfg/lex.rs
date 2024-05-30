use std::path::PathBuf;

use crate::utils::get_content;

pub struct DevGodotCfgLexCommand {
    /// The GodotCfg file to lex.
    file: PathBuf,
}

impl tapcli::Command for DevGodotCfgLexCommand {
    type Error = anyhow::Error;

    async fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        todo!()
    }

    async fn run(self) -> Result<Self::Output, Self::Error> {
        let content = get_content(self.file.as_path())?;
        let lexed = gdtk_godotcfg_parser::lexer(&content);

        for token in lexed {
            println!("{:?}", token);
        }

        Ok(())
    }
}
