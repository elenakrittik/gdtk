pub mod callbacks;
pub mod error;
#[cfg(test)]
mod tests;
pub mod token;

use gdtk_diag::Diagnostic;
use logos::Logos;

use crate::error::IntoDiag;
pub use crate::token::{Token, TokenKind};

pub type LexOutput<'a> = (Vec<Token<'a>>, Vec<Diagnostic>);

pub fn lex(input: &str) -> LexOutput {
    preprocess(TokenKind::lexer(input))
}

/// Combines tokens with their spans into a list of token structs,
/// as well as handles in-/de- dents.
fn preprocess<'a>(lexer: logos::Lexer<'a, TokenKind<'a>>) -> LexOutput<'a> {
    let mut tokens: Vec<Token<'a>> = vec![];
    let mut diags: Vec<Diagnostic> = vec![];

    for (result, span) in lexer.spanned() {
        match result {
            Ok(token) => {
                tokens.push(Token {
                    kind: token,
                    range: span,
                });
            }
            Err(err) => diags.push(err.into_diag(span)),
        }
    }

    tokens = generate_indents(tokens);

    (tokens, diags)
}

fn generate_indents(tokens: Vec<Token<'_>>) -> Vec<Token<'_>> {
    let mut stack: Vec<usize> = vec![0];
    let mut out = vec![];
    let mut new_line = false;

    let mut tokens = tokens.into_iter().peekable();

    while let Some(token) = tokens.next() {
        eprintln!("next token: {:?}", &token);
        eprintln!("current stack: {:?}", &stack);
        eprintln!("was previous token a newline: {:?}", &new_line);

        match token.kind {
            TokenKind::Newline => {
                let (token, indent_len) = if let Token { kind: TokenKind::Blank(b), .. } = tokens.peek() {
                    (iter.next(), b.len())
                } else {
                    (token, 0)
                };

                match indent_len.cmp(stack.last().unwrap()) {
                    std::cmp::Ordering::Greater => {
                        stack.push(indent_len);
                        out.push(token.transmute(TokenKind::Indent));
                    },
                    std::cmp::Ordering::Equal => out.push(token),
                    std::cmp::Ordering::Less => {
                        while stack.last().unwrap() > indent_len {
                            stach.pop();
                            out.push()
                        }
                    }
                }
            }
            _ => {
                out.push(token);
            }
        }
    }

    out
}
