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
    let mut tokens = tokens.into_iter().peekable();

    while let Some(token) = tokens.next() {
        match token.kind {
            TokenKind::Newline => {
                let (range, indent_len) = if tokens.peek().is_some_and(|t| t.kind.is_blank()) {
                    let token = tokens.next().unwrap();
                    let len = token.kind.as_blank().unwrap().len();

                    (token.range, len)
                } else {
                    (token.range.start..token.range.end, 0)
                };

                match indent_len.cmp(stack.last().unwrap()) {
                    std::cmp::Ordering::Greater => {
                        stack.push(indent_len);
                        out.push(token);
                        out.push(Token {
                            range,
                            kind: TokenKind::Indent,
                        });
                    }
                    std::cmp::Ordering::Equal => out.push(token),
                    std::cmp::Ordering::Less => {
                        let token = Token {
                            range,
                            kind: TokenKind::Dedent,
                        };

                        while stack.last().unwrap() > &indent_len {
                            stack.pop();
                            out.push(token.clone());
                        }
                    }
                }
            }
            TokenKind::Blank(_) => (),
            _ => {
                out.push(token);
            }
        }
    }

    out
}
