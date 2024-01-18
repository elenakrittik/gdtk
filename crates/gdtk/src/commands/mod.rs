use owo_colors::OwoColorize;

use crate::display::{print_error, ACTION};

pub mod godot;
pub mod parse;

pub fn run() -> anyhow::Result<()> {
    print_error(format!(
        "No command specififed. Run {} to view available options.",
        "`gdtk help`".style(*ACTION)
    ));
    Ok(())
}
