use crate::lexer::{cursor::Cursor, Token, TokenKind};

pub(super) struct Lexer<'a> {
    pub(super) source: &'a str,
    pub(super) cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            cursor: Cursor::new(source),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.start_token();

        let kind = if let Some(c) = self.cursor.peek() {
            #[rustfmt::skip]
            match c {
                '<' | '>' | '=' | '!' | '&' | '|' | '~' |
                '^' | '+' | '-' | '*' | '/' | '%' | '@' |
                '(' | ')' | '[' | ']' | '{' | '}' | ',' |
                ';' | '.' | ':' | '$' => {
                    self.lex_symbol()
                }
                '\n' | '\r' | '\t' | ' ' => {
                    self.lex_blanket()
                },
                '0'..='9' => {
                    self.lex_number(),
                }
                'A'..='Z' | 'a'..='z' | '_' => {
                    self.lex_identifier()
                }
                other => {
                    eprintln!("WARNING: skipping lexing char '{other}'");
                    self.cursor.next();
                    TokenKind::Indent
                },
            }
        } else {
            return None;
        };

        let span = self.cursor.current_span();

        Some(Token { span, kind })
    }
}
