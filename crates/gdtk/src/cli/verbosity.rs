use tracing::level_filters::LevelFilter;

pub struct Verbosity {
    verbosity: u8,
}

impl Verbosity {
    pub fn level(&self) -> anyhow::Result<LevelFilter> {
        match self.verbosity {
            0 => Ok(LevelFilter::INFO),
            1 => Ok(LevelFilter::ERROR),
            2 => Ok(LevelFilter::WARN),
            3 => Ok(LevelFilter::INFO),
            4 => Ok(LevelFilter::DEBUG),
            5 => Ok(LevelFilter::TRACE),
            _ => Err(anyhow::anyhow!(
                "Verbosity must be specified from 1 to 5 times."
            )),
        }
    }
}
