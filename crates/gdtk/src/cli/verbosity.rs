use clap::{ArgAction, Args};
use tracing::Level;

// N => level
// ----------
// 5 => trace
// 4 => debug
// 3 => info
// 2 => warn
// 1 => error

#[derive(Args)]
pub struct VerbosityArg {
    #[arg(
        short = 'v',
        long = "verbose",
        global = true,
        required = false,
        default_value_t = 3,
        action = ArgAction::Count,
        help = "Set log verbosity level.",
    )]
    verbosity: u8,
}

impl VerbosityArg {
    pub fn level(&self) -> anyhow::Result<Level> {
        match self.verbosity {
            1 => Ok(Level::ERROR),
            2 => Ok(Level::WARN),
            3 => Ok(Level::INFO),
            4 => Ok(Level::DEBUG),
            5 => Ok(Level::TRACE),
            // TODO: move this check into clap's value_parser
            _ => Err(anyhow::anyhow!(
                "Verbosity must be specified from 1 to 5 times."
            )),
        }
    }
}
