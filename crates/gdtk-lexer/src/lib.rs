pub mod callbacks;
pub mod error;
#[cfg(test)]
mod tests;
pub mod token;

use std::iter::Peekable;

use logos::Logos;

pub use crate::token::{Token, TokenKind};

pub fn lex<'a>(input: &'a str) -> impl Iterator<Item = Token<'a>> {
    let tokens = TokenKind::lexer(input)
        .spanned()
        .filter_map(|(result, range)| result.ok().map(|kind| Token { range, kind }))
        .peekable();

    generate_indents(tokens).into_iter()
}

fn generate_indents<'a>(mut tokens: Peekable<impl Iterator<Item = Token<'a>>>) -> Vec<Token<'a>> {
    let mut stack: Vec<usize> = vec![0];
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
            TokenKind::Blank(_) | TokenKind::Comment(_) => (),
            _ => out.push(token),
        }
    }

    out
}
