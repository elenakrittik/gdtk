#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "rustc", feature(let_chains))]

pub mod diagnostic;
pub mod protocol;
pub mod utils;
pub mod visualizers;

pub type Span = std::ops::Range<usize>;

pub use diagnostic::{Diagnostic, Highlight, Severity};
pub use protocol::Visualizer;
