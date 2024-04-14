//! A GDScript 2.0 parser.

#![feature(decl_macro, stmt_expr_attributes, type_alias_impl_trait)]

use std::iter::Peekable;

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
pub mod parser;
pub mod statement;
pub mod statements;
#[cfg(test)]
pub mod test_utils;
pub mod utils;
pub mod values;
pub mod variables;

pub type Parser<'a, I> = crate::parser::Parser<Peekable<I>>;

/// Parse the result of lexing a GDScript source code file.
pub fn parse_file<'a>(tokens: impl Iterator<Item = Token<'a>>) -> Result<ASTFile<'a>, Error> {
    let mut body: CodeBlock<'_> = vec![];
    let mut parser = crate::parser::Parser::new(tokens);

    while let Some(token) = parser.peek() {
        match token.kind {
            // ignore leftover dedents from parsing parenthesized lambdas
            TokenKind::Newline | TokenKind::Semicolon | TokenKind::Dedent => {
                parser.next();
            }
            _ => body.push(parse_statement(&mut parser)),
        }
    }

    Ok(ASTFile { body })
}
