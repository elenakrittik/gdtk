
use crate::lexer::{TokenKind, lexer::Lexer};

impl<'a> Lexer<'a> {
    pub(super) fn lex_identifier(&mut self) -> TokenKind<'a> {
        #[cfg(debug_assertions)]
        assert!(self.cursor.peek().is_some_and(|c| c.is_ascii_alphabetic() || c == &'_'));

        self.cursor.next();

        while self.cursor.peek().is_some_and(|c| c.is_ascii_alphanumeric() || c == &'_') {
            self.cursor.next();
        }

        match self.cursor.current_text() {
            "null" => TokenKind::Null,
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "not" => TokenKind::Not,
            "as" => TokenKind::As,
            "await" => TokenKind::Await,
            "in" => TokenKind::In,
            // TODO: figure out how to do this
            "not in" => TokenKind::NotIn,
            "is" => TokenKind::Is,
            "if" => TokenKind::If,
            "elif" => TokenKind::Elif,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "pass" => TokenKind::Pass,
            "return" => TokenKind::Return,
            "match" => TokenKind::Match,
            "assert" => TokenKind::Assert,
            "breakpoint" => TokenKind::Breakpoint,
            "class" => TokenKind::Class,
            "class_name" => TokenKind::ClassName,
            "const" => TokenKind::Const,
            "enum" => TokenKind::Enum,
            "extends" => TokenKind::Extends,
            "func" => TokenKind::Func,
            "signal" => TokenKind::Signal,
            "static" => TokenKind::Static,
            "var" => TokenKind::Var,
            "when" => TokenKind::When,
            "namespace" => TokenKind::Namespace,
            "trait" => TokenKind::Trait,
            "yield" => TokenKind::Yield,
            other => TokenKind::Identifier(other),
        }
    }
}
