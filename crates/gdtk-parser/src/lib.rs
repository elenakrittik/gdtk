#![feature(decl_macro, stmt_expr_attributes)]

use gdtk_ast::poor::{ASTFile, CodeBlock};
use gdtk_lexer::{token::TokenKind, LexOutput};

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
pub mod test_utils;
pub mod utils;
pub mod values;
pub mod variables;

/// Parse the result of lexing a GDScript source code file.
pub fn parse_file(lexed: LexOutput) -> Result<ASTFile, Error> {
    let (tokens, _diags) = lexed;

    let mut body: CodeBlock<'_> = vec![];
    let mut iter = tokens.into_iter().peekable();

    while let Some(token) = iter.peek() {
        match token.kind {
            // ignore leftover dedents from parsing paremthwsized lambdas
            TokenKind::Newline | TokenKind::Dedent => {
                iter.next();
            }
            _ => body.push(parse_statement(&mut iter)),
        }
    }

    Ok(ASTFile { body })
}
