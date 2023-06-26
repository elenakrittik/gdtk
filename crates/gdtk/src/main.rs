use gdtk::commands::parse::run as parse;
use gdtk::display::{action, print_error};

fn main() -> anyhow::Result<()> {
    let cli = gdtk::cli::parse();

    match &cli.command {
        Some(gdtk::cli::Commands::Parse { file }) => parse(file)?,
        None => {
            print_error(format!(
                "no command specified. Run {} to view help message.",
                action("`gdtk help`")
            ));
        }
    }

    Ok(())
}
