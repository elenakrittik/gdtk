#![feature(decl_macro)]

pub mod callbacks;
pub mod error;
#[cfg(test)]
mod tests;
pub mod token;
pub mod utils;

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
    let mut stack: Vec<u64> = vec![0];
    let mut out = vec![];
    let mut treat_as_indent = false;

    let mut tokens = tokens.into_iter().peekable();

    while let Some(token) = tokens.next() {
        match token.kind {
            TokenKind::Blank(b) => {
                if !treat_as_indent {
                    out.push(token);
                    continue;
                }

                treat_as_indent = false;

                let len = &(b.len() as u64);

                match len.cmp(stack.last().unwrap()) {
                    std::cmp::Ordering::Less => {
                        while len < stack.last().unwrap() {
                            stack.pop();
                            out.push(Token {
                                range: token.range.clone(),
                                kind: TokenKind::Dedent,
                            });
                        }

                        if len > stack.last().unwrap() {
                            out.push(Token {
                                range: token.range.clone(),
                                kind: TokenKind::Indent,
                            });
                        }
                    }
                    std::cmp::Ordering::Equal => (),
                    std::cmp::Ordering::Greater => {
                        stack.push(b.len() as u64);
                        out.push(token.transmute(TokenKind::Indent));
                    }
                }
            }
            TokenKind::Newline => {
                eprintln!("found newline");
                if !matches!(
                    tokens.peek(),
                    Some(Token {
                        kind: TokenKind::Blank(_) | TokenKind::Newline | TokenKind::Comment(_),
                        ..
                    })
                ) {
                    eprintln!("next token is not a blank or a newline (meaning we are now at the top-most level)");
                    while stack.last().unwrap() != &0 {
                        stack.pop();
                        out.push(Token {
                            range: token.range.clone(),
                            kind: TokenKind::Dedent,
                        });
                    }
                }

                eprintln!("next token is a blank or a newline, continue trying to match block");
                treat_as_indent = true;
                out.push(token);
            },
            TokenKind::Colon => {
                out.push(Token { kind: TokenKind::Colon, range: token.range.clone() });

                match tokens.peek().unwrap() {
                    Token { kind: TokenKind::Newline | TokenKind::Comment(_), .. } => {
                        () // block starts as usual, on a new line; nothing to do
                    },
                    Token { kind: TokenKind::Blank(_), .. } => {
                        // skipping blanket, as it
                        // 1. isnt significant in this place
                        // 2. blocks us from peeking farther
                        tokens.next();

                        match tokens.peek().unwrap() {
                            Token { kind: TokenKind::Newline | TokenKind::Comment(_), .. } => {
                                () // t'was just a trailing space, same case as the outer match
                            },
                            _ => {
                                // blank-prefixed inline code, inserting indent
                                stack.push(stack.last().unwrap() + 1);
                                out.push(Token { kind: TokenKind::Indent, range: token.range.clone() });
                            }
                        }
                    },
                    _ => {
                        // blank-prefixed inline code, inserting indent
                        stack.push(stack.last().unwrap() + 1);
                        out.push(Token { kind: TokenKind::Indent, range: token.range.clone() });
                    },
                }
            },
            _ => {
                out.push(token);
                treat_as_indent = false;
            }
        }
    }

    out
}
