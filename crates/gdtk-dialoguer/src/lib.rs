//! This is a fork of [dialoguer]. Use the official crate instead.

#![feature(iter_repeat_n)]
#![feature(type_alias_impl_trait)]
#![feature(stmt_expr_attributes)]

#[cfg(feature = "completion")]
pub use completion::Completion;
pub use console;
#[cfg(feature = "editor")]
pub use edit::Editor;
pub use error::{Error, Result};
#[cfg(feature = "history")]
pub use history::{BasicHistory, History};
#[cfg(feature = "fuzzy-select")]
pub use prompts::fuzzy_select::FuzzySelect;
pub use prompts::prompt::Prompt;
pub use validate::{InputValidator, PasswordValidator};

#[cfg(feature = "completion")]
mod completion;
#[cfg(feature = "editor")]
mod edit;
mod error;
#[cfg(feature = "history")]
mod history;
mod prompts;
pub mod theme;
mod validate;
