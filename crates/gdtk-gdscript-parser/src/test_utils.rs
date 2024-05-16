use gdtk_ast::{ASTExpr, ASTExprKind, ASTPassStmt, ASTStatement};
use crate::lexer::{Token, TokenKind};

use crate::Parser;

pub(crate) const PASS_STMT: ASTStatement = ASTStatement::Pass(ASTPassStmt { span: 0..0 });

pub(crate) fn create_parser(input: &str) -> Parser<impl Iterator<Item = Token<'_>>> {
    crate::parser::Parser::new(crate::lexer::lex(input))
}

pub(crate) fn next_kind<'a>(
    parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
) -> TokenKind<'a> {
    parser.next().unwrap().kind
}

pub(crate) fn make_ident(ident: &str) -> ASTExpr<'_> {
    ASTExpr {
        kind: ASTExprKind::Identifier(ident),
        span: 0..0,
    }
}

pub(crate) fn make_number(num: u64) -> ASTExpr<'static> {
    ASTExpr {
        kind: ASTExprKind::Number(num),
        span: 0..0,
    }
}

pub(crate) fn make_string(string: &str) -> ASTExpr<'_> {
    ASTExpr {
        kind: ASTExprKind::String(string),
        span: 0..0,
    }
}
