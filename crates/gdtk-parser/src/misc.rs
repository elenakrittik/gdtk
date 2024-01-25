use gdtk_lexer::{Token, TokenKind};
use gdtk_ast::poor::ASTAnnotation;

use crate::utils::{expect_blank_prefixed, collect_args};

pub fn parse_annotation<'a, T>(iter: &mut T) -> ASTAnnotation<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(i), i);
    let arguments = collect_args!(iter, TokenKind::OpeningParenthesis, TokenKind::ClosingParenthesis);

    ASTAnnotation {
        identifier,
        arguments,
    }
}
