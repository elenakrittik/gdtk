
use crate::lexer::{TokenKind, lexer::Lexer};

impl<'a> Lexer<'a> {
    pub(super) fn lex_blanket(&mut self) -> TokenKind<'a> {
        match self.cursor.next().unwrap() {
            '\n' => TokenKind::Newline,
            '\r' => {
                if self.cursor.peek().is_some_and(|c| c == &'\n') {
                    self.cursor.next();
                }

                TokenKind::Newline
            },
            ' ' | '\t' => {
                while self.cursor.peek().is_some_and(|c| c == &' ' || c == &'\t') {
                    self.cursor.next();
                }

                TokenKind::Blank(self.cursor.current_text())
            },
            _ => unreachable!(),
        }
    }
}
