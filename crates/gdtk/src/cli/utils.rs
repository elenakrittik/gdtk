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

    fn next_godot_version(&mut self, pool: Vec<Versioning>) -> anyhow::Result<Versioning> {
        let prompt = |items: Vec<Versioning>| {
            Prompt::builder()
                .with_question("Select Godot version")
                .with_items(items.iter().collect::<Vec<_>>())
                .with_state(PromptState::default())
                .with_action(
                    cliui::Key::Char('d'),
                    cliui::Action {
                        description: "Show in-development versions.",
                        callback: |prompt| {
                            prompt.state.dev = !prompt.state.dev;

                            Ok(())
                        },
                    },
                )
                .build()
                .interact()
                .map(|o| o.map(|v| v.to_owned()))
        };

        if let Some(input) = self.next_value_maybe()? {
            let mut matches = coerce_version(new_godot_versioning(&input)?, pool)?;

            let version = if matches.len() == 1 {
                matches.swap_remove(0)
            } else {
                prompt(matches)?.unwrap()
            };

            Ok(version)
        } else {
            Ok(prompt(pool)?.unwrap())
        }
    }
}

#[derive(Default)]
struct PromptState {
    dev: bool,
    mono: bool,
}
