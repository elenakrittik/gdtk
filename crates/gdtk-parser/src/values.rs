use gdtk_lexer::Token;
use gdtk_ast::poor::{ASTValue, DictValue};
use crate::utils::{expect_blank_prefixed, next_non_blank, collect_args_raw};

pub fn parse_value<'a, T>(iter: &mut T, mut token: Option<Token<'a>>) -> ASTValue<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    if token.is_none() {
        token = Some(next_non_blank!(iter));
    }

    match token.unwrap() {
        Token::Identifier(s) => {
            match next_non_blank!(iter) {
                Token::OpeningParenthesis => ASTValue::Call(
                    Box::new(ASTValue::Identifier(s)),
                    collect_args_raw!(iter, Token::ClosingParenthesis),
                ),
                Token::Newline => ASTValue::Identifier(s),
                // TODO: prop access
                other => panic!("unexpected {other:?}, expected parenthesis"),
            }
        }
        Token::Integer(i) => ASTValue::Number(i),
        Token::BinaryInteger(i) => ASTValue::Number(i as i64),
        Token::HexInteger(i) => ASTValue::Number(i as i64),
        Token::ScientificFloat(f) => ASTValue::Float(f),
        Token::Float(f) => ASTValue::Float(f),
        Token::String(s) => ASTValue::String(s),
        Token::StringName(s) => ASTValue::StringName(s),
        Token::Node(s) => ASTValue::Node(s),
        Token::UniqueNode(s) => ASTValue::UniqueNode(s),
        Token::NodePath(s) => ASTValue::NodePath(s),
        Token::Boolean(b) => ASTValue::Boolean(b),
        Token::OpeningBracket => ASTValue::Array(collect_args_raw!(iter, Token::ClosingBracket)),
        Token::OpeningBrace => ASTValue::Dictionary(parse_dictionary(iter)),
        Token::Minus => match parse_value(iter, None) {
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
        Token::ClosingBrace => (), // empty dict
        Token::Identifier(s) => parse_lua_dict(iter, &mut vec, ASTValue::String(s)),
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
    expect_blank_prefixed!(iter, Token::Assignment, ());
    let first_val = parse_value(iter, None);
    vec.push((first_key, first_val));

    let mut expect_comma = true; // just got our pair, expect a comma

    loop {
        match next_non_blank!(iter) {
            Token::Comma => {
                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            }
            Token::Identifier(s) => {
                expect_blank_prefixed!(iter, Token::Assignment, ());
                vec.push((ASTValue::String(s), parse_value(iter, None)));
                expect_comma = true;
            }
            Token::ClosingBrace => break,
            other => panic!("unexpected {other:?}"),
        }
    }
}

pub fn parse_python_dict<'a, T>(iter: &mut T, vec: &mut DictValue<'a>, first_key: ASTValue<'a>)
where
    T: Iterator<Item = Token<'a>>,
{
    expect_blank_prefixed!(iter, Token::Colon, ());
    let first_val = parse_value(iter, None);
    vec.push((first_key, first_val));

    let mut expect_comma = true; // just got our pair, expect a comma

    loop {
        match next_non_blank!(iter) {
            Token::Comma => {
                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            }
            Token::ClosingBrace => break,
            other => {
                let key = parse_value(iter, Some(other));
                expect_blank_prefixed!(iter, Token::Colon, ());
                vec.push((key, parse_value(iter, None)));
                expect_comma = true;
            }
        }
    }
}
