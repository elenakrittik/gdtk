use clap::{ArgAction, Args};
use tracing::level_filters::LevelFilter;

#[derive(Args)]
pub struct VerbosityArg {
    #[arg(
        short = 'v',
        long = "verbose",
        global = true,
        action = ArgAction::Count,
        help = "Set log verbosity level.",
    )]
    verbosity: u8,
}

impl VerbosityArg {
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
