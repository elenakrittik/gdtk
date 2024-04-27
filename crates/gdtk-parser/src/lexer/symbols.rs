use super::TokenKind;
use crate::lexer::lexer::Lexer;

impl<'a> Lexer<'a> {
    pub(super) fn lex_symbol(&mut self) -> TokenKind<'a> {
        let mut symbol = match self.cursor.next().unwrap() {
            '<' => {
                if self.cursor.peek().is_some_and(|c| c == &'<') {
                    TokenKind::BitwiseShiftLeft
                } else {
                    TokenKind::LessThan
                }
            }
            '>' => {
                if self.cursor.peek().is_some_and(|c| c == &'>') {
                    TokenKind::BitwiseShiftRight
                } else {
                    TokenKind::GreaterThan
                }
            }
            '=' => TokenKind::Assignment,
            '!' => TokenKind::SymbolizedNot,
            '&' => TokenKind::BitwiseAnd,
            '|' => TokenKind::BitwiseOr,
            '~' => TokenKind::BitwiseNot,
            '^' => TokenKind::BitwiseXor,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => {
                if self.cursor.peek().is_some_and(|c| c == &'*') {
                    TokenKind::Power
                } else {
                    TokenKind::Multiply
                }
            }
            '/' => TokenKind::Divide,
            '%' => TokenKind::Remainder,
            '@' => TokenKind::Annotation,
            '(' => TokenKind::OpeningParenthesis,
            ')' => TokenKind::ClosingParenthesis,
            '[' => TokenKind::OpeningBracket,
            ']' => TokenKind::ClosingBracket,
            '{' => TokenKind::OpeningBrace,
            '}' => TokenKind::ClosingBrace,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            '.' => TokenKind::Period,
            ':' => TokenKind::Colon,
            '$' => TokenKind::Dollar,
            _ => unreachable!(),
        };

        if self.cursor.peek().is_some_and(|c| c == &'=')
            && let Some(assigned) = assigned(&symbol)
        {
            symbol = assigned;
        }

        symbol
    }
}

const fn assigned(kind: &TokenKind<'_>) -> Option<TokenKind<'static>> {
    Some(match kind {
        TokenKind::LessThan => TokenKind::LessThanOrEqual,
        TokenKind::GreaterThan => TokenKind::GreaterThanOrEqual,
        TokenKind::SymbolizedNot => TokenKind::NotEqual,
        TokenKind::BitwiseAnd => TokenKind::BitwiseAndAssignment,
        TokenKind::BitwiseOr => TokenKind::BitwiseOrAssignment,
        TokenKind::BitwiseNot => TokenKind::BitwiseNotAssignment,
        TokenKind::BitwiseXor => TokenKind::BitwiseXorAssignment,
        TokenKind::BitwiseShiftLeft => TokenKind::BitwiseShiftLeftAssignment,
        TokenKind::BitwiseShiftRight => TokenKind::BitwiseShiftRightAssignment,
        TokenKind::Plus => TokenKind::PlusAssignment,
        TokenKind::Minus => TokenKind::MinusAssignment,
        TokenKind::Multiply => TokenKind::MultiplyAssignment,
        TokenKind::Power => TokenKind::PowerAssignment,
        TokenKind::Divide => TokenKind::DivideAssignment,
        TokenKind::Remainder => TokenKind::RemainderAssignment,
        TokenKind::Assignment => TokenKind::Equal,
        _ => return None,
    })
}
