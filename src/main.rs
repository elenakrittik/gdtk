mod parser;

use crate::parser::parse_file;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
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
    Parse {
        #[arg(short, long)]
        file: Option<String>,
    },
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Some(Commands::Format { files }) => {
            if !files.is_empty() {
                println!(
                    "{} {} {}",
                    "Formatting".green(),
                    files.len().to_string().blue(),
                    "files!".green(),
                );
            } else {
                println!("{}", "No files found".red(),);
            }
        }
        Some(Commands::Parse { file }) => {
            let fs = file.as_ref().unwrap();
            if !fs.is_empty() {
                println!("{} {}{}", "Parsing".green(), fs.blue(), ".".green(),);
            } else {
                println!("{}", "No file specified.".red());
            }
        }
        None => {
            println!("{}", "No command bruh".red());
        }
    }
}
