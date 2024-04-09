pub mod callbacks;
pub mod error;
#[cfg(test)]
mod tests;
pub mod token;

use std::iter::Peekable;

use logos::Logos;

pub use crate::token::{Token, TokenKind};

pub fn lex(input: &str) -> impl Iterator<Item = Token<'_>> {
    let tokens = TokenKind::lexer(input)
        .spanned()
        .filter_map(|(result, range)| result.ok().map(|kind| Token { range, kind }))
        .peekable();

    generate_indents(tokens).into_iter()
}

// I wish there was a way to make it 100% iterator-based, but last time i tried it turned out
// clunky and even slower. Help appreciated
fn generate_indents<'a>(mut tokens: Peekable<impl Iterator<Item = Token<'a>>>) -> Vec<Token<'a>> {
    // TODO: check if smallvec will improve perf here
    let mut stack: Vec<usize> = vec![0];
    // TODO: check if pre-allocating will improve perf here
    let mut out = vec![];

    while let Some(token) = tokens.next() {
        match token.kind {
            TokenKind::Newline => {
                if tokens.peek().is_some_and(|t| t.kind.is_newline()) {
                    continue;
                }

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
                        // TODO: check if squashing into .extend will improve perf here
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

                        // TODO: check if squashing into std::iter::repeat_n will improve perf here
                        while stack.last().unwrap() > &indent_len {
                            stack.pop();
                            out.push(token.clone());
                        }
                    }
                }
            }
            TokenKind::Blank(_) | TokenKind::Comment(_) => (),
            _ => out.push(token),
        }
    }

    out
}
