use cliui::Prompt;
use gdtk_gvm::{utils::coerce_version, versions::Versioning};

use crate::{
    cli::{missing, unknown},
    utils::new_godot_versioning,
};

pub(crate) trait ParserExt {
    fn next_value(&mut self) -> anyhow::Result<String>;
    fn next_value_maybe(&mut self) -> anyhow::Result<Option<String>>;
    fn next_godot_version(&mut self, pool: Vec<Versioning>) -> anyhow::Result<Versioning>;
}

impl ParserExt for tapcli::Parser {
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

    fn next_godot_version(&mut self, mut pool: Vec<Versioning>) -> anyhow::Result<Versioning> {
        let prompt = |items: &[Versioning]| {
            Prompt::builder()
                .with_question("Select Godot version")
                .with_items(items.iter())
                .build()
                .interact()
        };

        if let Some(input) = self.next_value_maybe()? {
            let mut matches = coerce_version(new_godot_versioning(&input)?, pool)?;

            let idx = if matches.len() == 1 {
                0
            } else {
                prompt(&matches)?.unwrap()
            };

            Ok(matches.swap_remove(idx))
        } else {
            let idx = prompt(&pool)?.unwrap();

            Ok(pool.swap_remove(idx))
        }
    }
}
