use colored::Colorize;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Format {
        /// lists test values
        #[arg(short, long)]
        files: Vec<String>,
    },
}

fn main() {
    let cli: Cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    match &cli.command {
        Some(Commands::Format { files }) => {
            if !files.is_empty() {
                println!("Formatting {} files!", files.len().to_string().green());
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

    // Continued program logic goes here...
}