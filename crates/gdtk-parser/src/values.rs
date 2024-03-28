use std::iter::Peekable;

use gdtk_ast::poor::{ASTBinaryOp, ASTUnaryOp, ASTValue, DictValue};
use gdtk_lexer::{Token, TokenKind};

use crate::{
    functions::parse_func,
    utils::{
        collect_args, collect_args_raw, expect_blank_prefixed, next_non_blank, peek_non_blank,
    },
};

pub fn parse_value<'a, T>(iter: &mut Peekable<T>, mut token: Option<Token<'a>>) -> ASTValue<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    if token.is_none() {
        token = Some(next_non_blank!(iter));
    }

    let token = token.unwrap();

    let val = match token.kind {
        TokenKind::Identifier(s) => ASTValue::Identifier(s),
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
        TokenKind::OpeningBracket => {
            ASTValue::Array(collect_args_raw!(iter, TokenKind::ClosingBracket))
        }
        TokenKind::OpeningBrace => ASTValue::Dictionary(parse_dictionary(iter)),
        TokenKind::Minus => {
            let value = parse_value(iter, None);
            ASTValue::UnaryExpr(ASTUnaryOp::Minus, Box::new(value))
        }
        TokenKind::Comment(c) => ASTValue::Comment(c),
        TokenKind::Func => ASTValue::Lambda(parse_func(iter, true)),
        _ => panic!("unknown or unsupported expression: {token:?}"),
    };

    maybe_op(iter, val)
}

pub fn parse_dictionary<'a, T>(iter: &mut Peekable<T>) -> DictValue<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let mut vec: DictValue<'a> = vec![];

    match next_non_blank!(iter) {
        Token {
            kind: TokenKind::ClosingBrace,
            ..
        } => (), // empty dict
        Token {
            kind: TokenKind::Identifier(s),
            ..
        } => parse_lua_dict(iter, &mut vec, ASTValue::String(s)),
        other => {
            let first_key = parse_value(iter, Some(other));
            parse_python_dict(iter, &mut vec, first_key);
        }
    }

    vec
}

pub fn parse_lua_dict<'a, T>(
    iter: &mut Peekable<T>,
    vec: &mut DictValue<'a>,
    first_key: ASTValue<'a>,
) where
    T: Iterator<Item = Token<'a>>,
{
    expect_blank_prefixed!(iter, TokenKind::Assignment, ());
    let first_val = parse_value(iter, None);
    vec.push((first_key, first_val));

    let mut expect_comma = true; // just got our pair, expect a comma

    loop {
        match next_non_blank!(iter) {
            Token {
                kind: TokenKind::Comma,
                ..
            } => {
                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            }
            Token {
                kind: TokenKind::Identifier(s),
                ..
            } => {
                expect_blank_prefixed!(iter, TokenKind::Assignment, ());
                vec.push((ASTValue::String(s), parse_value(iter, None)));
                expect_comma = true;
            }
            Token {
                kind: TokenKind::ClosingBrace,
                ..
            } => break,
            other => panic!("unexpected {other:?}"),
        }
    }
}

pub fn parse_python_dict<'a, T>(
    iter: &mut Peekable<T>,
    vec: &mut DictValue<'a>,
    first_key: ASTValue<'a>,
) where
    T: Iterator<Item = Token<'a>>,
{
    expect_blank_prefixed!(iter, TokenKind::Colon, ());
    let first_val = parse_value(iter, None);
    vec.push((first_key, first_val));

    let mut expect_comma = true; // just got our pair, expect a comma

    loop {
        match next_non_blank!(iter) {
            Token {
                kind: TokenKind::Comma,
                ..
            } => {
                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            }
            Token {
                kind: TokenKind::ClosingBrace,
                ..
            } => break,
            other => {
                let key = parse_value(iter, Some(other));
                expect_blank_prefixed!(iter, TokenKind::Colon, ());
                vec.push((key, parse_value(iter, None)));
                expect_comma = true;
            }
        }
    }
}

pub fn maybe_op<'a, T>(iter: &mut Peekable<T>, left: ASTValue<'a>) -> ASTValue<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let op = match peek_non_blank(iter) {
        Some(Token { kind: TokenKind::Plus, .. }) => Some(ASTBinaryOp::Plus),
        Some(Token { kind: TokenKind::Minus, .. }) => Some(ASTBinaryOp::Minus),
        Some(Token { kind: TokenKind::Greater, .. }) => Some(ASTBinaryOp::Greater),
        Some(Token { kind: TokenKind::GreaterOrEqual, .. }) => Some(ASTBinaryOp::GreaterOrEqual),
        Some(Token { kind: TokenKind::Less, .. }) => Some(ASTBinaryOp::Less),
        Some(Token { kind: TokenKind::LessOrEqual, .. }) => Some(ASTBinaryOp::LessOrEqual),
        Some(Token { kind: TokenKind::Period, .. }) => Some(ASTBinaryOp::PropertyAccess),
        Some(Token { kind: TokenKind::Multiply, .. }) => Some(ASTBinaryOp::Multiply),
        Some(Token { kind: TokenKind::Divide, .. }) => Some(ASTBinaryOp::Divide),
        Some(Token { kind: TokenKind::Equal, .. }) => Some(ASTBinaryOp::Equal),
        Some(Token { kind: TokenKind::NotEqual, .. }) => Some(ASTBinaryOp::NotEqual),
        Some(Token { kind: TokenKind::And | TokenKind::SymbolizedAnd, .. }) => Some(ASTBinaryOp::And),
        Some(Token { kind: TokenKind::Or | TokenKind::SymbolizedOr, .. }) => Some(ASTBinaryOp::Or),
        Some(Token { kind: TokenKind::Not | TokenKind::SymbolizedNot, .. }) => Some(ASTBinaryOp::Not),
        Some(Token { kind: TokenKind::BitwiseAnd, .. }) => Some(ASTBinaryOp::BitwiseAnd),
        Some(Token { kind: TokenKind::BitwiseOr, .. }) => Some(ASTBinaryOp::BitwiseOr),
        Some(Token { kind: TokenKind::BitwiseNot, .. }) => Some(ASTBinaryOp::BitwiseNot),
        Some(Token { kind: TokenKind::BitwiseXor, .. }) => Some(ASTBinaryOp::BitwiseXor),
        Some(Token { kind: TokenKind::BitwiseShiftLeft, .. }) => Some(ASTBinaryOp::BitwiseShiftLeft),
        Some(Token { kind: TokenKind::BitwiseShiftRight, .. }) => Some(ASTBinaryOp::BitwiseShiftRight),
        Some(Token { kind: TokenKind::Power, .. }) => Some(ASTBinaryOp::Power),
        Some(Token { kind: TokenKind::Remainder, .. }) => Some(ASTBinaryOp::Remainder),
        Some(Token { kind: TokenKind::As, .. }) => Some(ASTBinaryOp::TypeCast),
        Some(Token { kind: TokenKind::Is, .. }) => Some(ASTBinaryOp::TypeCheck),
        Some(Token { kind: TokenKind::In, .. }) => Some(ASTBinaryOp::Contains),
        Some(Token { kind: TokenKind::Range, .. }) => Some(ASTBinaryOp::Range),
        Some(Token { kind: TokenKind::OpeningParenthesis, .. }) => {
            return ASTValue::Call(
                Box::new(left),
                collect_args!(
                    iter,
                    TokenKind::OpeningParenthesis,
                    TokenKind::ClosingParenthesis
                ),
            );
        }
        Some(Token { kind: TokenKind::OpeningBrace, .. }) => {
            return ASTValue::Subscript(Box::new(left), Box::new(parse_value(iter, None)))
        }
        _ => None,
    };

    if op.is_none() {
        return left;
    } else {
        iter.next();
    }

    let right = parse_value(iter, None);

    // TODO: precedence
    ASTValue::BinaryExpr(Box::new(left), op.unwrap(), Box::new(right))
}

// TODO: use this by default for all values

pub fn maybe_uop<'a, T>(iter: &mut Peekable<T>) -> ASTValue<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    let op = match peek_non_blank(iter) {
        Some(Token { kind: TokenKind::Plus, .. }) => Some(ASTUnaryOp::Plus),
        Some(Token { kind: TokenKind::Minus, .. }) => Some(ASTUnaryOp::Minus),
        Some(Token { kind: TokenKind::Await, .. }) => Some(ASTUnaryOp::Await),
        _ => None,
    };

    if let Some(_op) = op {
        parse_value(iter, None)
    } else {
        iter.next();
        ASTValue::UnaryExpr(op.unwrap(), Box::new(parse_value(iter, None)))
    }
}
