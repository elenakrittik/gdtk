use gdtk_ast::{ASTExpr, ASTExprKind};
use gdtk_lexer::{Token, TokenKind};

use crate::Parser;

pub(crate) fn create_parser(input: &str) -> Parser<impl Iterator<Item = Token<'_>>> {
    crate::parser::Parser::new(gdtk_lexer::lex(input))
}

pub(crate) fn next_kind<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> TokenKind<'a> {
    parser.next().unwrap().kind
}

pub(crate) fn make_ident(ident: &str) -> ASTExpr<'_> {
    ASTExpr {
        kind: ASTExprKind::Identifier(ident),
        range: 0..0,
    }
}

pub(crate) fn make_number(num: u64) -> ASTExpr<'static> {
    ASTExpr {
        kind: ASTExprKind::Number(num),
        range: 0..0,
    }
}

pub(crate) fn make_string(string: &str) -> ASTExpr<'_> {
    ASTExpr {
        kind: ASTExprKind::String(string),
        range: 0..0,
    }
}
