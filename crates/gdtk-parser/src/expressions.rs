use std::iter::Peekable;

use gdtk_ast::poor::{ASTBinaryOp, ASTUnaryOp, ASTValue};
use gdtk_lexer::{Token, TokenKind};

use crate::{
    functions::parse_func,
    utils::{delemited_by, expect, peek_non_blank},
    values::parse_dictionary,
};

pub fn parse_expr<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTValue<'a> {
    let initial_value = parse_expr_with_ops(iter);

    let mut values_and_ops = vec![];

    #[rustfmt::skip]
    while let Some(op) = match peek_non_blank(iter).map(|t| &t.kind) {
        Some(TokenKind::Plus) => Some(ASTBinaryOp::Plus),
        Some(TokenKind::Minus) => Some(ASTBinaryOp::Minus),
        Some(TokenKind::Greater) => Some(ASTBinaryOp::Greater),
        Some(TokenKind::GreaterOrEqual) => Some(ASTBinaryOp::GreaterOrEqual),
        Some(TokenKind::Less) => Some(ASTBinaryOp::Less),
        Some(TokenKind::LessOrEqual) => Some(ASTBinaryOp::LessOrEqual),
        Some(TokenKind::Period) => Some(ASTBinaryOp::PropertyAccess),
        Some(TokenKind::Multiply) => Some(ASTBinaryOp::Multiply),
        Some(TokenKind::Divide) => Some(ASTBinaryOp::Divide),
        Some(TokenKind::Equal) => Some(ASTBinaryOp::Equal),
        Some(TokenKind::NotEqual) => Some(ASTBinaryOp::NotEqual),
        Some(TokenKind::And | TokenKind::SymbolizedAnd) => Some(ASTBinaryOp::And),
        Some(TokenKind::Or | TokenKind::SymbolizedOr) => Some(ASTBinaryOp::Or),
        Some(TokenKind::BitwiseAnd) => Some(ASTBinaryOp::BitwiseAnd),
        Some(TokenKind::BitwiseOr) => Some(ASTBinaryOp::BitwiseOr),
        Some(TokenKind::BitwiseXor) => Some(ASTBinaryOp::BitwiseXor),
        Some(TokenKind::BitwiseShiftLeft) => Some(ASTBinaryOp::BitwiseShiftLeft),
        Some(TokenKind::BitwiseShiftRight) => Some(ASTBinaryOp::BitwiseShiftRight),
        Some(TokenKind::Power) => Some(ASTBinaryOp::Power),
        Some(TokenKind::Remainder) => Some(ASTBinaryOp::Remainder),
        Some(TokenKind::As) => Some(ASTBinaryOp::TypeCast),
        Some(TokenKind::Is) => Some(ASTBinaryOp::TypeCheck),
        Some(TokenKind::In) => Some(ASTBinaryOp::Contains),
        Some(TokenKind::NotIn) => Some(ASTBinaryOp::NotContains),
        Some(TokenKind::Range) => Some(ASTBinaryOp::Range),
        Some(TokenKind::Assignment) => Some(ASTBinaryOp::Assignment),
        Some(TokenKind::PlusAssignment) => Some(ASTBinaryOp::PlusAssignment),
        Some(TokenKind::MinusAssignment) => Some(ASTBinaryOp::MinusAssignment),
        Some(TokenKind::MultiplyAssignment) => Some(ASTBinaryOp::MultiplyAssignment),
        Some(TokenKind::PowerAssignment) => Some(ASTBinaryOp::PowerAssignment),
        Some(TokenKind::DivideAssignment) => Some(ASTBinaryOp::DivideAssignment),
        Some(TokenKind::RemainderAssignment) => Some(ASTBinaryOp::RemainderAssignment),
        Some(TokenKind::BitwiseAndAssignment) => Some(ASTBinaryOp::BitwiseAndAssignment),
        Some(TokenKind::BitwiseOrAssignment) => Some(ASTBinaryOp::BitwiseNotAssignment),
        Some(TokenKind::BitwiseNotAssignment) => Some(ASTBinaryOp::BitwiseNotAssignment),
        Some(TokenKind::BitwiseXorAssignment) => Some(ASTBinaryOp::BitwiseXorAssignment),
        Some(TokenKind::BitwiseShiftLeftAssignment) => Some(ASTBinaryOp::BitwiseShiftLeftAssignment),
        Some(TokenKind::BitwiseShiftRightAssignment) => Some(ASTBinaryOp::BitwiseShiftRightAssignment),
        _ => None,
    } {
        iter.next();

        let value = parse_expr_with_ops(iter);
        values_and_ops.push((op, value));
    }

    let expr = prec::Expression::new(initial_value, values_and_ops);

    binary_climber().process(&expr, &()).unwrap()
}

/// Parses a value taking into account possible prefix and postfix OPs
pub fn parse_expr_with_ops<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTValue<'a> {
    let mut prefix_ops = vec![];

    while let Some(op) = match peek_non_blank(iter).map(|t| &t.kind) {
        Some(TokenKind::Plus) => Some(ASTUnaryOp::Identity),
        Some(TokenKind::Minus) => Some(ASTUnaryOp::Minus),
        Some(TokenKind::Await) => Some(ASTUnaryOp::Await),
        Some(TokenKind::BitwiseNot) => Some(ASTUnaryOp::BitwiseNot),
        Some(TokenKind::Not | TokenKind::SymbolizedNot) => Some(ASTUnaryOp::Not),
        None => panic!("expected expression"),
        _ => None,
    } {
        iter.next();
        prefix_ops.push(op);
    }

    let mut value = parse_expr_without_ops(iter);

    // Calls/subscriptions have higher precedence, i.e. `-get_num()` should be parsed as `-(get_num())`
    while let Some(op) = match peek_non_blank(iter).map(|t| &t.kind) {
        Some(TokenKind::OpeningParenthesis) => Some(ASTBinaryOp::Call),
        Some(TokenKind::OpeningBracket) => Some(ASTBinaryOp::Subscript),
        _ => None,
    } {
        iter.next();
        match op {
            ASTBinaryOp::Call => {
                value = ASTValue::BinaryExpr(
                    Box::new(value),
                    op,
                    Box::new(ASTValue::Array(delemited_by(
                        iter,
                        TokenKind::Comma,
                        &[TokenKind::ClosingParenthesis],
                        parse_expr,
                    ))),
                );
                expect!(iter, TokenKind::ClosingParenthesis);
            }
            ASTBinaryOp::Subscript => {
                value = ASTValue::BinaryExpr(Box::new(value), op, Box::new(parse_expr(iter)));
                expect!(iter, TokenKind::ClosingBracket);
            }
            _ => unreachable!(),
        }
    }

    for op in prefix_ops {
        value = ASTValue::UnaryExpr(op, Box::new(value));
    }

    value
}

/// Parses a "clean" value, without checking for possible prefix or postfix OPs
pub fn parse_expr_without_ops<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ASTValue<'a> {
    #[rustfmt::skip]
    match &peek_non_blank(iter).expect("unexpected EOF").kind {
        TokenKind::Identifier(_) => ASTValue::Identifier(iter.next().unwrap().kind.into_identifier().unwrap()),
        TokenKind::Integer(_) => ASTValue::Number(iter.next().unwrap().kind.into_integer().unwrap()),
        TokenKind::BinaryInteger(_) => ASTValue::Number(iter.next().unwrap().kind.into_binary_integer().unwrap() as i64),
        TokenKind::HexInteger(_) => ASTValue::Number(iter.next().unwrap().kind.into_hex_integer().unwrap() as i64),
        TokenKind::Float(_) => ASTValue::Float(iter.next().unwrap().kind.into_float().unwrap()),
        TokenKind::ScientificFloat(_) => ASTValue::Float(iter.next().unwrap().kind.into_scientific_float().unwrap()),
        TokenKind::String(_) => ASTValue::String(iter.next().unwrap().kind.into_string().unwrap()),
        TokenKind::StringName(_) => ASTValue::StringName(iter.next().unwrap().kind.into_string_name().unwrap()),
        TokenKind::Node(_) => ASTValue::Node(iter.next().unwrap().kind.into_node().unwrap()),
        TokenKind::UniqueNode(_) => ASTValue::UniqueNode(iter.next().unwrap().kind.into_unique_node().unwrap()),
        TokenKind::NodePath(_) => ASTValue::NodePath(iter.next().unwrap().kind.into_node_path().unwrap()),
        TokenKind::Boolean(_) => ASTValue::Boolean(iter.next().unwrap().kind.into_boolean().unwrap()),
        TokenKind::Comment(_) => ASTValue::Comment(iter.next().unwrap().kind.into_comment().unwrap()),
        TokenKind::Func => ASTValue::Lambda(parse_func(iter, true)),
        TokenKind::OpeningBracket => {
            iter.next();
            let value = ASTValue::Array(delemited_by(
                iter,
                TokenKind::Comma,
                &[TokenKind::ClosingBracket],
                parse_expr,
            ));
            expect!(iter, TokenKind::ClosingBracket);

            value
        }
        TokenKind::OpeningBrace => ASTValue::Dictionary(parse_dictionary(iter)),
        TokenKind::OpeningParenthesis => {
            iter.next();
            let value = ASTValue::Group(Box::new(parse_expr(iter)));
            expect!(iter, TokenKind::ClosingParenthesis);
            value
        },
        other => panic!("unknown or unsupported expression: {other:?}"),
    }
}

fn binary_climber<'a>() -> prec::Climber<ASTBinaryOp, ASTValue<'a>, ASTValue<'a>, ()> {
    fn handler<'a>(
        lhs: ASTValue<'a>,
        op: ASTBinaryOp,
        rhs: ASTValue<'a>,
        _ctx: &(),
    ) -> Result<ASTValue<'a>, ()> {
        Ok(ASTValue::BinaryExpr(Box::new(lhs), op, Box::new(rhs)))
    }

    // TODO: figure out the correct associations (pain)
    prec::Climber::new(
        vec![
            prec::Rule::new(ASTBinaryOp::Subscript, prec::Assoc::Left),
            prec::Rule::new(ASTBinaryOp::PropertyAccess, prec::Assoc::Left),
            prec::Rule::new(ASTBinaryOp::Call, prec::Assoc::Left),
            prec::Rule::new(ASTBinaryOp::TypeCheck, prec::Assoc::Left),
            prec::Rule::new(ASTBinaryOp::Power, prec::Assoc::Left),
            prec::Rule::new(ASTBinaryOp::Multiply, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::Divide, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::Remainder, prec::Assoc::Left),
            prec::Rule::new(ASTBinaryOp::Plus, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::Minus, prec::Assoc::Left),
            prec::Rule::new(ASTBinaryOp::BitwiseShiftLeft, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::BitwiseShiftRight, prec::Assoc::Left),
            prec::Rule::new(ASTBinaryOp::Less, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::LessOrEqual, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::Greater, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::GreaterOrEqual, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::Equal, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::NotEqual, prec::Assoc::Left),
            prec::Rule::new(ASTBinaryOp::Contains, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::NotContains, prec::Assoc::Left),
            prec::Rule::new(ASTBinaryOp::And, prec::Assoc::Left),
            prec::Rule::new(ASTBinaryOp::Or, prec::Assoc::Left),
            prec::Rule::new(ASTBinaryOp::TypeCast, prec::Assoc::Left),
            prec::Rule::new(ASTBinaryOp::Assignment, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::PlusAssignment, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::MinusAssignment, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::MultiplyAssignment, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::DivideAssignment, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::PowerAssignment, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::RemainderAssignment, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::BitwiseAndAssignment, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::BitwiseOrAssignment, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::BitwiseXorAssignment, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::BitwiseShiftLeftAssignment, prec::Assoc::Left)
                | prec::Rule::new(ASTBinaryOp::BitwiseShiftRightAssignment, prec::Assoc::Left),
        ],
        handler,
    )
}
