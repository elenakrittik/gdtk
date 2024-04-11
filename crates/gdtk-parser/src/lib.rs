//! A GDScript 2.0 parser.

#![feature(decl_macro, stmt_expr_attributes)]

use gdtk_ast::poor::{ASTFile, CodeBlock};
use gdtk_lexer::{token::TokenKind, Token};

pub use crate::error::Error;
use crate::statement::parse_statement;

pub mod block;
pub mod classes;
pub mod error;
pub mod expressions;
pub mod functions;
pub mod match_;
pub mod misc;
pub mod statement;
pub mod statements;
#[cfg(test)]
pub mod test_utils;
pub mod utils;
pub mod values;
pub mod variables;

/// Parse the result of lexing a GDScript source code file.
pub fn parse_file<'a>(tokens: impl Iterator<Item = Token<'a>>) -> Result<ASTFile<'a>, Error> {
    let mut body: CodeBlock<'_> = vec![];
    let mut iter = tokens.peekable();

    while let Some(token) = iter.peek() {
        match token.kind {
            // ignore leftover dedents from parsing parenthesized lambdas
            TokenKind::Newline | TokenKind::Dedent => {
                iter.next();
            }
            _ => body.push(parse_statement(&mut iter)),
        }
    }

    Ok(ASTFile { body })
}
