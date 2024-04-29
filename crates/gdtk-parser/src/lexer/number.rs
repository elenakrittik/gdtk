use crate::lexer::{TokenKind, lexer::Lexer};

enum NumKind {
    Decimal,
    Hexadecimal,
    Binary,
}

impl<'a> Lexer<'a> {
    pub(super) fn lex_number(&mut self) -> TokenKind<'a> {
        match self.cursor.next().unwrap() {
            '0' => match self.cursor.peek() {
                Some('x') => todo!(),
                Some('b') => todo!(),
                Some(other) => todo!(),
                None => TokenKind::Number(0),
            },
            _ => {
                while self.cursor.peek().is_some_and(|c| c.is_ascii_numeric()) {
                    self.cursor.next();
                }

                TokenKind::Number(self.cursor.current_text().parse())
            }
        }
    }
}
