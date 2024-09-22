use cliui::Prompt;
use gdtk_gvm::VersionManager;

use crate::cli::{missing, unknown};

#[extend::ext(name = ParserExt)]
pub impl tapcli::Parser {
    fn next_value(&mut self) -> anyhow::Result<String> {
        Ok(match self.next_value_maybe()? {
            Some(value) => value,
            None => missing!("a value"),
        })
    }

    fn next_value_maybe(&mut self) -> anyhow::Result<Option<String>> {
        Ok(match self.next() {
            Some(tapcli::Arg::Value(value)) => Some(value),
            None => None,
            other => unknown!(other),
        })
    }
}

pub fn prompt_local_version(manager: &VersionManager) -> anyhow::Result<&str> {
    let available_versions = manager.versions();

    let (Some(version), _) = Prompt::builder()
        .with_question("Select version")
        .with_items(available_versions)
        .build()
        .interact()?
    else {
        anyhow::bail!("Command cancelled.");
    };

    Ok(version)
}
