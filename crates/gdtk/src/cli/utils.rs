use crate::cli::{missing, unknown};

pub(crate) trait ParserExt {
    fn next_value(&mut self) -> anyhow::Result<String>;
    fn next_value_maybe(&mut self) -> anyhow::Result<Option<String>>;
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
            Some(tapcli::Arg::Value(version)) => Some(version),
            None => None,
            other => unknown!(other),
        })
    }
}
