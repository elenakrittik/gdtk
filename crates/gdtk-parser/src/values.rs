use std::iter::Peekable;

use gdtk_ast::poor::{ASTBinaryOp, ASTUnaryOp, ASTValue, DictValue};
use gdtk_lexer::{Token, TokenKind};

use crate::utils::{
    collect_args, collect_args_raw, expect_blank_prefixed, next_non_blank, peek_non_blank,
};

pub fn parse_value<'a, T>(iter: &mut Peekable<T>, mut token: Option<Token<'a>>) -> ASTValue<'a>
where
    T: Iterator<Item = Token<'a>>,
{
    if token.is_none() {
        token = Some(next_non_blank!(iter));
    }

    let val = match token.unwrap().kind {
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
        other => panic!("unknown or unsupported expression: {other:?}"),
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
    let op = match &peek_non_blank!(iter).kind {
        &TokenKind::Plus => Some(ASTBinaryOp::Plus),
        &TokenKind::Minus => Some(ASTBinaryOp::Minus),
        &TokenKind::Greater => Some(ASTBinaryOp::Greater),
        &TokenKind::GreaterOrEqual => Some(ASTBinaryOp::GreaterOrEqual),
        &TokenKind::Less => Some(ASTBinaryOp::Less),
        &TokenKind::LessOrEqual => Some(ASTBinaryOp::LessOrEqual),
        &TokenKind::Period => Some(ASTBinaryOp::PropertyAccess),
        &TokenKind::Multiply => Some(ASTBinaryOp::Multiply),
        &TokenKind::Divide => Some(ASTBinaryOp::Divide),
        &TokenKind::Equal => Some(ASTBinaryOp::Equal),
        &TokenKind::NotEqual => Some(ASTBinaryOp::NotEqual),
        &TokenKind::And | &TokenKind::SymbolizedAnd => Some(ASTBinaryOp::And),
        &TokenKind::Or | &TokenKind::SymbolizedOr => Some(ASTBinaryOp::Or),
        &TokenKind::Not | &TokenKind::SymbolizedNot => Some(ASTBinaryOp::Not),
        &TokenKind::BitwiseAnd => Some(ASTBinaryOp::BitwiseAnd),
        &TokenKind::BitwiseOr => Some(ASTBinaryOp::BitwiseOr),
        &TokenKind::BitwiseNot => Some(ASTBinaryOp::BitwiseNot),
        &TokenKind::BitwiseXor => Some(ASTBinaryOp::BitwiseXor),
        &TokenKind::BitwiseShiftLeft => Some(ASTBinaryOp::BitwiseShiftLeft),
        &TokenKind::BitwiseShiftRight => Some(ASTBinaryOp::BitwiseShiftRight),
        &TokenKind::Power => Some(ASTBinaryOp::Power),
        &TokenKind::Remainder => Some(ASTBinaryOp::Remainder),
        &TokenKind::As => Some(ASTBinaryOp::TypeCast),
        &TokenKind::Is => Some(ASTBinaryOp::TypeCheck),
        &TokenKind::In => Some(ASTBinaryOp::Contains),
        &TokenKind::Range => Some(ASTBinaryOp::Range),
        &TokenKind::OpeningParenthesis => {
            return ASTValue::Call(
                Box::new(left),
                collect_args!(
                    iter,
                    TokenKind::OpeningParenthesis,
                    TokenKind::ClosingParenthesis
                ),
            );
        }
        &TokenKind::OpeningBrace => {
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
    let op = match peek_non_blank!(iter).kind {
        TokenKind::Plus => Some(ASTUnaryOp::Plus),
        TokenKind::Minus => Some(ASTUnaryOp::Minus),
        TokenKind::Await => Some(ASTUnaryOp::Await),
        _ => None,
    };

    if let Some(_op) = op {
        parse_value(iter, None)
    } else {
        iter.next();
        ASTValue::UnaryExpr(op.unwrap(), Box::new(parse_value(iter, None)))
    }
}
