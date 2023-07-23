//! Utility functions for consistent output displaying.

use owo_colors::{OwoColorize, Style};

lazy_static::lazy_static! {
    pub static ref ERROR: Style = Style::new().red();
    pub static ref ACTION: Style = Style::new().green().bold();
    pub static ref THING: Style = Style::new().yellow();
}

/// Prints an error using format_error and eprintln
#[inline]
pub fn print_error(text: String) {
    eprintln!("{}", format_error(text));
}

/// Format error text.
#[inline]
pub fn format_error(text: String) -> String {
    format!("{} {}", "error:".style(*ERROR), text)
}
