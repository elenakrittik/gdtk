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

fn generate_indents<'a>(tokens: Vec<Token<'a>>) -> Vec<Token<'a>> {
    let mut stack: Vec<u64> = vec![0];
    let mut out = vec![];
    let mut new_line = false;

    let mut tokens = tokens.into_iter().peekable();

    while let Some(token) = tokens.next() {
        match token.kind {
            TokenKind::Blank(b) => {
                if !new_line {
                    out.push(token);
                    continue;
                }

                new_line = false;

                let len = &(b.len() as u64);

                match len.cmp(stack.last().unwrap()) {
                    std::cmp::Ordering::Less => {
                        while len < stack.last().unwrap() {
                            stack.pop();
                            out.push(Token { range: token.range.clone(), kind: TokenKind::Dedent });
                        }

                        if len > stack.last().unwrap() {
                            out.push(Token { range: token.range.clone(), kind: TokenKind::Indent });
                        }
                    },
                    std::cmp::Ordering::Equal => (),
                    std::cmp::Ordering::Greater => {
                        stack.push(b.len() as u64);
                        out.push(token.transmute(TokenKind::Indent));
                    },
                }
            },
            TokenKind::Newline => {
                if !matches!(tokens.peek(), Some(Token { kind: TokenKind::Blank(_), .. })) {
                    while stack.last().unwrap() != &0 {
                        stack.pop();
                        out.push(Token { range: token.range.clone(), kind: TokenKind::Dedent });
                    }
                }

                new_line = true;
            },
            _ => {
                out.push(token);
                new_line = false;
            },
        }
    }

    out
}
