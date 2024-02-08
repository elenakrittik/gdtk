use std::iter::Peekable;

use gdtk_ast::poor::ASTAnnotation;
use gdtk_lexer::{Token, TokenKind};

use crate::utils::{collect_args, expect_blank_prefixed};

pub fn parse_annotation<'a, T>(iter: &mut Peekable<T>) -> ASTAnnotation<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(i), i);
    let arguments = collect_args!(
        iter,
        TokenKind::OpeningParenthesis,
        TokenKind::ClosingParenthesis
    );

    ASTAnnotation {
        identifier,
        arguments,
    }
}
