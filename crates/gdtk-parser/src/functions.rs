use gdtk_ast::poor::{ASTFunction, ASTFunctionParameter, CodeBlock, ASTStatement};
use gdtk_lexer::{Token, TokenKind};

use crate::{variables::parse_const, values::parse_value};
use crate::utils::{expect, expect_blank_prefixed, next_non_blank, parse_idtydef};

pub fn parse_func<'a, T>(iter: &mut T) -> ASTFunction<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let identifier = expect_blank_prefixed!(iter, TokenKind::Identifier(s), s);
    expect_blank_prefixed!(iter, TokenKind::OpeningParenthesis, ());
    let mut parameters = vec![];

    loop {
        let mut expect_comma = true;
        let mut break_ = false;

        let (identifier, infer_type, typehint, default) = parse_idtydef!(
            iter,
            TokenKind::Comma => { dbg!("got comma"); expect_comma = false; },
            TokenKind::ClosingParenthesis => { break_ = true; dbg!("got end paren"); },
        );

        parameters.push(ASTFunctionParameter {
            identifier,
            infer_type,
            typehint,
            default,
        });

        if break_ {
            break;
        }

        if expect_comma {
            match next_non_blank!(iter) {
                Token {
                    kind: TokenKind::Comma,
                    ..
                } => (),
                Token {
                    kind: TokenKind::ClosingParenthesis,
                    ..
                } => break,
                other => panic!("expected comma or closing parenthesis, found {other:?}"),
            }
        }
    }

    expect_blank_prefixed!(iter, TokenKind::Colon, ());
    expect_blank_prefixed!(iter, TokenKind::Newline, ());

    let (body, exit_indent) = parse_func_body(iter);

    ASTFunction {
        identifier,
        parameters,
        body: vec![],
    }
}

pub fn parse_func_body<'a, T>(iter: &mut T) -> (CodeBlock<'a>, usize)
where
    T: Iterator<Item = Token<'a>>,
{
    let indent = expect!(iter, TokenKind::Blank(b), b).chars().count();
    let mut exit_indent: usize = usize::MAX;
    let mut body = vec![];

    loop {
        match iter.next() {
            Some(token) => match token {
                Token { kind: TokenKind::Const, .. } => body.push(ASTStatement::Variable(parse_const(iter))),
                Token { kind: TokenKind::Pass, .. } => body.push(ASTStatement::Pass),
                Token { kind: TokenKind::Continue, .. } => body.push(ASTStatement::Continue),
                Token { kind: TokenKind::Break, .. } => body.push(ASTStatement::Break),
                Token { kind: TokenKind::Return, .. } => body.push(ASTStatement::Return(parse_value(iter, None))),
                _ => panic!("idk unsupported or smth just f u"),
            },
            None => {
                if body.len() <= 0 {
                    panic!("expected indented block, found EOF");
                }

                break;
            },
        }

        expect_blank_prefixed!(iter, TokenKind::Newline, ());

        match iter.next() {
            Some(token) => match token {
                // something indented
                Token { kind: TokenKind::Blank(b), .. } => {
                    if b.len() < indent { // dedent
                        exit_indent = b.len();
                        break;
                    } else if b.len() == indent { // current block continues
                        ()
                    } else { // extraneous indent
                        panic!("unexpected indentation level");
                    }
                },
                other => panic!("unexpected {other:?}"),
            },
            None => break, // end of file
        }
    }

    (body, exit_indent)
}
