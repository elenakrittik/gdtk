use cliui::Prompt;
use gdtk_gvm::{types::LocalVersion, VersionManager};

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

pub fn prompt_local_version(manager: &VersionManager) -> anyhow::Result<&LocalVersion> {
    let available_versions = manager.installed();

    let (Some(version), _) = Prompt::builder()
        .with_question("Select version")
        .with_items(available_versions.iter().collect::<Vec<_>>()) // FIXME: remove vec requirement, somehow?
        .build()
        .interact()?
    else {
        anyhow::bail!("Command cancelled.");
    };

    Ok(version)
}
