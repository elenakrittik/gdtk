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

pub fn noqa_comments(input: &str) -> ahash::AHashMap<usize, Vec<&str>> {
    let mut map = ahash::AHashMap::<usize, Vec<&str>>::new();
    let mut line = 0usize;

    let tokens = TokenKind::lexer(input).filter_map(Result::ok);

    for token in tokens {
        match token {
            TokenKind::Newline => line += 1,
            TokenKind::Comment(comm) if comm.trim_end().starts_with("noqa") => {
                if let Some(v) = map.get_mut(&line) {
                    v.push(comm);
                } else {
                    map.insert(line, vec![comm]);
                }
            },
            _ => (),
        }
    }

    map
}
