//! This is a fork of [dialoguer]. Use the official crate instead.

#![deny(clippy::all)]

#[cfg(feature = "completion")]
pub use completion::Completion;
pub use console;
#[cfg(feature = "editor")]
pub use edit::Editor;
pub use error::{Error, Result};
#[cfg(feature = "history")]
pub use history::{BasicHistory, History};
use paging::Paging;
#[cfg(feature = "fuzzy-select")]
pub use prompts::fuzzy_select::FuzzySelect;
#[cfg(feature = "password")]
pub use prompts::password::Password;
pub use prompts::{
    confirm::Confirm, input::Input, multi_select::MultiSelect, select::Select, sort::Sort,
};
pub use validate::{InputValidator, PasswordValidator};

#[cfg(feature = "completion")]
mod completion;
#[cfg(feature = "editor")]
mod edit;
mod error;
#[cfg(feature = "history")]
mod history;
mod paging;
mod prompts;
pub mod theme;
mod validate;
