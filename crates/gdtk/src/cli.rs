use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Parse {
        #[arg(short, long)]
        file: String,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}
