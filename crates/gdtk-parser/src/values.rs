use gdtk_lexer::{Token, TokenKind};
use gdtk_ast::poor::{ASTValue, DictValue};
use crate::utils::{expect_blank_prefixed, next_non_blank, collect_args_raw};

pub fn parse_value<'a, T>(iter: &mut T, mut token: Option<Token<'a>>) -> ASTValue<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    if token.is_none() {
        token = Some(next_non_blank!(iter));
    }

    match token.unwrap().kind {
        TokenKind::Identifier(s) => {
            match next_non_blank!(iter) {
                Token { kind: TokenKind::OpeningParenthesis, .. } => ASTValue::Call(
                    Box::new(ASTValue::Identifier(s)),
                    collect_args_raw!(iter, TokenKind::ClosingParenthesis),
                ),
                Token { kind: TokenKind::Newline, .. } => ASTValue::Identifier(s),
                // TODO: prop access
                other => panic!("unexpected {other:?}, expected parenthesis"),
            }
        },
        TokenKind::Integer(i) => ASTValue::Number(i),
        TokenKind::BinaryInteger(i) => ASTValue::Number(i as i64),
        TokenKind::HexInteger(i) => ASTValue::Number(i as i64),
        TokenKind::ScientificFloat(f) => ASTValue::Float(f),
        TokenKind::Float(f) => ASTValue::Float(f),
        TokenKind::String(s) => ASTValue::String(s),
        TokenKind::StringName(s) => ASTValue::StringName(s),
        TokenKind::Node(s) => ASTValue::Node(s),
        TokenKind::UniqueNode(s) => ASTValue::UniqueNode(s),
        TokenKind::NodePath(s) => ASTValue::NodePath(s),
        TokenKind::Boolean(b) => ASTValue::Boolean(b),
        TokenKind::OpeningBracket => ASTValue::Array(collect_args_raw!(iter, TokenKind::ClosingBracket)),
        TokenKind::OpeningBrace => ASTValue::Dictionary(parse_dictionary(iter)),
        TokenKind::Minus => match parse_value(iter, None) {
            ASTValue::Number(n) => ASTValue::Number(-n),
            ASTValue::Float(f) => ASTValue::Float(f),
            _ => panic!("unary minus is supported for numbers and float only"),
        },
        other => panic!("unknown or unsupported expression: {other:?}"),
    }
}

pub fn parse_dictionary<'a, T>(iter: &mut T) -> DictValue<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let mut vec: DictValue<'a> = vec![];

    match next_non_blank!(iter) {
        Token { kind: TokenKind::ClosingBrace, .. } => (), // empty dict
        Token { kind: TokenKind::Identifier(s), .. } => parse_lua_dict(iter, &mut vec, ASTValue::String(s)),
        other => {
            let first_key = parse_value(iter, Some(other));
            parse_python_dict(iter, &mut vec, first_key);
        }
    }

    vec
}

pub fn parse_lua_dict<'a, T>(iter: &mut T, vec: &mut DictValue<'a>, first_key: ASTValue<'a>)
where
    T: Iterator<Item = Token<'a>>,
{
    expect_blank_prefixed!(iter, TokenKind::Assignment, ());
    let first_val = parse_value(iter, None);
    vec.push((first_key, first_val));

    let mut expect_comma = true; // just got our pair, expect a comma

    loop {
        match next_non_blank!(iter) {
            Token { kind: TokenKind::Comma, .. } => {
                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            }
            Token { kind: TokenKind::Identifier(s), .. } => {
                expect_blank_prefixed!(iter, TokenKind::Assignment, ());
                vec.push((ASTValue::String(s), parse_value(iter, None)));
                expect_comma = true;
            }
            Token { kind: TokenKind::ClosingBrace, .. } => break,
            other => panic!("unexpected {other:?}"),
        }
    }
}

pub fn parse_python_dict<'a, T>(iter: &mut T, vec: &mut DictValue<'a>, first_key: ASTValue<'a>)
where
    T: Iterator<Item = Token<'a>>,
{
    expect_blank_prefixed!(iter, TokenKind::Colon, ());
    let first_val = parse_value(iter, None);
    vec.push((first_key, first_val));

    let mut expect_comma = true; // just got our pair, expect a comma

    loop {
        match next_non_blank!(iter) {
            Token { kind: TokenKind::Comma, .. } => {
                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            }
            Token { kind: TokenKind::ClosingBrace, .. } => break,
            other => {
                let key = parse_value(iter, Some(other));
                expect_blank_prefixed!(iter, TokenKind::Colon, ());
                vec.push((key, parse_value(iter, None)));
                expect_comma = true;
            }
        }
    }
}
