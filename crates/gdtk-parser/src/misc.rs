use gdtk_lexer::Token;
use gdtk_ast::poor::ASTAnnotation;

use crate::utils::{expect_blank_prefixed, collect_args};

pub fn parse_annotation<'a, T>(iter: &mut T) -> ASTAnnotation<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let identifier = expect_blank_prefixed!(iter, Token::Identifier(i), i);
    let arguments = collect_args!(iter, Token::OpeningParenthesis, Token::ClosingParenthesis);

    ASTAnnotation {
        identifier,
        arguments,
    }
}
