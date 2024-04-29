#![feature(lazy_cell)]

pub mod callbacks;
pub mod error;
#[cfg(test)]
mod tests;
pub mod token;

use std::{cell::LazyCell, iter::Peekable};

use logos::Logos;
use regex::Regex;
use token::CommentLexer;

pub use crate::token::{Token, TokenKind};

pub fn lex(input: &str) -> impl Iterator<Item = Token<'_>> {
    let tokens = TokenKind::lexer(input)
        .spanned()
        .filter_map(|(result, span)| result.ok().map(|kind| Token { span, kind }))
        .peekable();

    generate_indents(tokens).into_iter()
}

// I wish there was a way to make it 100% iterator-based, but last time i tried it turned out
// clunky and even slower. Help appreciated
fn generate_indents<'a>(mut tokens: Peekable<impl Iterator<Item = Token<'a>>>) -> Vec<Token<'a>> {
    let mut stack: Vec<usize> = vec![0];
    let mut out = vec![];

    while let Some(token) = tokens.next() {
        match token.kind {
            TokenKind::Newline => {
                if tokens.peek().is_some_and(|t| t.kind.is_newline()) {
                    continue;
                }

                let (span, indent_len) = if tokens.peek().is_some_and(|t| t.kind.is_blank()) {
                    let token = tokens.next().unwrap();
                    let len = token.kind.as_blank().unwrap().len();

                    (token.span, len)
                } else {
                    (token.span.start..token.span.end, 0)
                };

                match indent_len.cmp(stack.last().unwrap()) {
                    std::cmp::Ordering::Greater => {
                        stack.push(indent_len);
                        out.push(token);
                        out.push(Token {
                            span,
                            kind: TokenKind::Indent,
                        });
                    }
                    std::cmp::Ordering::Equal => out.push(token),
                    std::cmp::Ordering::Less => {
                        let token = Token {
                            span,
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

    if stack.last().unwrap() > &0 {
        let token = Token {
            span: 0..0, // should be fine?
            kind: TokenKind::Dedent,
        };

        while stack.last().unwrap() > &0 {
            stack.pop();
            out.push(token.clone());
        }
    }

    out
}

pub fn noqas(input: &str) -> ahash::AHashMap<usize, Vec<&str>> {
    let mut map = ahash::AHashMap::<usize, Vec<&str>>::new();
    let mut line = 0usize;

    let tokens = CommentLexer::lexer(input).filter_map(Result::ok);

    for token in tokens {
        match token {
            CommentLexer::Newline => line += 1,
            CommentLexer::Comment(comm) if comm.contains("noqa") => {
                map.insert(line, find_noqas(comm));
            }
            _ => (),
        }
    }

    map
}

fn find_noqas(input: &str) -> Vec<&str> {
    const REGEX: LazyCell<Regex> =
        LazyCell::new(|| Regex::new(r"noqa:[ \t]*([a-zA-Z-]+)").unwrap());

    REGEX
        .captures_iter(input)
        .filter_map(|c| c.get(1).map(|m| m.as_str()))
        .collect()
}
