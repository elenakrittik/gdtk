mod parser;

use crate::parser::parse_file;
use clap::{Parser, Subcommand};
use colored::Colorize;
use pest::Parser as PestParser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

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
        file: String,
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
            let fs = file;
            if !fs.is_empty() {
                println!("{} {}{}", "Parsing".green(), fs.blue(), ".".green(),);
                match parse_file(fs.to_string()) {
                    Ok(content) => println!("{}", content),
                    Err(_) => println!("{}", "error: unable to read file contents".red()),
                };
            } else {
                println!("{} {}", "error:".red(), "no file specified.");
            }
        }
        None => {
            println!(
                "{} {} {} {}",
                "error:".red(),
                "no command specified. Run",
                "`gdtk help`".green(),
                "to view help message.",
            );
        }
    }
}
