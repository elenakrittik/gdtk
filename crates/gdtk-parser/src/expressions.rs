use std::iter::Peekable;

use gdtk_ast::poor::{ASTBinaryOp, ASTPostfixOp, ASTPrefixOp, ASTValue};
use gdtk_lexer::{Token, TokenKind};
use pratt::{Affix, Associativity, PrattParser, Precedence};

use crate::{
    functions::parse_func,
    utils::{delemited_by, expect},
    values::{parse_array, parse_dictionary},
};

pub fn parse_expr<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ASTValue<'a> {
    ExprParser.parse(parse_expr_impl(iter).into_iter()).unwrap()
}

fn parse_expr_impl<'a>(iter: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> Vec<ExprIR<'a>> {
    let mut result = parse_expr_with_ops(iter);

    #[rustfmt::skip]
    while let Some(op) = match iter.peek().map(|t| &t.kind) {
        Some(TokenKind::Plus) => Some(ASTBinaryOp::Add),
        Some(TokenKind::Minus) => Some(ASTBinaryOp::Substract),
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
        Some(TokenKind::OpeningParenthesis) => Some(ASTBinaryOp::Call),
        Some(TokenKind::OpeningBracket) => Some(ASTBinaryOp::Subscript),
        _ => None,
    } {
        iter.next();
        result.push(ExprIR::Binary(op));

        let value = parse_expr_with_ops(iter);
        result.extend(value);

        match op {
            ASTBinaryOp::Call => expect!(iter, TokenKind::ClosingParenthesis),
            ASTBinaryOp::Subscript => expect!(iter, TokenKind::ClosingBracket),
            _ => (),
        }
    }

    result
}

/// Parses a value taking into account possible prefix and postfix OPs
fn parse_expr_with_ops<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> Vec<ExprIR<'a>> {
    let mut result = vec![];

    while let Some(op) = match iter.peek().map(|t| &t.kind) {
        Some(TokenKind::Plus) => Some(ASTPrefixOp::Identity),
        Some(TokenKind::Minus) => Some(ASTPrefixOp::Negation),
        Some(TokenKind::Await) => Some(ASTPrefixOp::Await),
        Some(TokenKind::BitwiseNot) => Some(ASTPrefixOp::BitwiseNot),
        Some(TokenKind::Not | TokenKind::SymbolizedNot) => Some(ASTPrefixOp::Not),
        None => panic!("expected expression"),
        _ => None,
    } {
        iter.next();
        result.push(ExprIR::Prefix(op));
    }

    result.push(parse_expr_without_ops(iter));

    result
}

/// Parses a "clean" value, without checking for possible prefix or postfix OPs
fn parse_expr_without_ops<'a>(
    iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> ExprIR<'a> {
    #[rustfmt::skip]
    let value = match &iter.peek().expect("unexpected EOF").kind {
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
        TokenKind::OpeningBracket => ASTValue::Array(parse_array(iter)),
        TokenKind::OpeningBrace => ASTValue::Dictionary(parse_dictionary(iter)),
        TokenKind::OpeningParenthesis => {
            iter.next();

            let values = delemited_by(
                iter,
                TokenKind::Comma,
                &[TokenKind::ClosingParenthesis],
                parse_expr_impl,
            ).into_iter().flatten().collect();

            expect!(iter, TokenKind::ClosingParenthesis);
            
            return ExprIR::Group(values);
        },
        other => panic!("unknown or unsupported expression: {other:?}"),
    };

    ExprIR::Primary(value)
}

#[derive(Debug, enum_as_inner::EnumAsInner)]
enum ExprIR<'a> {
    Prefix(ASTPrefixOp),
    Binary(ASTBinaryOp),
    Group(Vec<ExprIR<'a>>),
    Primary(ASTValue<'a>),
}

struct ExprParser;

impl<'a, I> pratt::PrattParser<I> for ExprParser
where
    I: Iterator<Item = ExprIR<'a>>,
{
    type Error = pratt::NoError;
    type Input = ExprIR<'a>;
    type Output = ASTValue<'a>;

    fn query(&mut self, input: &Self::Input) -> Result<Affix, Self::Error> {
        Ok(match input {
            ExprIR::Primary(_) => Affix::Nilfix,
            ExprIR::Group(_) => Affix::Nilfix,
            ExprIR::Binary(ASTBinaryOp::Subscript) => Affix::Postfix(Precedence(22)),
            ExprIR::Binary(ASTBinaryOp::PropertyAccess) => Affix::Infix(Precedence(21), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::Call) => Affix::Postfix(Precedence(20)),
            ExprIR::Prefix(ASTPrefixOp::Await) => Affix::Prefix(Precedence(19)),
            ExprIR::Binary(ASTBinaryOp::TypeCheck) => Affix::Infix(Precedence(18), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::Power) => Affix::Infix(Precedence(17), Associativity::Left),

            ExprIR::Prefix(ASTPrefixOp::BitwiseNot) => Affix::Prefix(Precedence(16)),

            ExprIR::Prefix(ASTPrefixOp::Identity) => Affix::Prefix(Precedence(15)),
            ExprIR::Prefix(ASTPrefixOp::Negation) => Affix::Prefix(Precedence(15)),

            ExprIR::Binary(ASTBinaryOp::Multiply) => Affix::Infix(Precedence(14), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::Divide) => Affix::Infix(Precedence(14), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::Remainder) => Affix::Infix(Precedence(14), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::Add) => Affix::Infix(Precedence(13), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::Substract) => Affix::Infix(Precedence(13), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::BitwiseShiftLeft) => Affix::Infix(Precedence(12), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::BitwiseShiftRight) => Affix::Infix(Precedence(12), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::BitwiseAnd) => Affix::Infix(Precedence(11), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::BitwiseXor) => Affix::Infix(Precedence(10), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::BitwiseOr) => Affix::Infix(Precedence(9), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::Equal) => Affix::Infix(Precedence(8), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::NotEqual) => Affix::Infix(Precedence(8), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::Less) => Affix::Infix(Precedence(8), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::LessOrEqual) => Affix::Infix(Precedence(8), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::Greater) => Affix::Infix(Precedence(8), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::GreaterOrEqual) => Affix::Infix(Precedence(8), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::Contains) => Affix::Infix(Precedence(7), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::NotContains) => Affix::Infix(Precedence(7), Associativity::Left),

            ExprIR::Prefix(ASTPrefixOp::Not) => Affix::Prefix(Precedence(6)),

            ExprIR::Binary(ASTBinaryOp::And) => Affix::Infix(Precedence(5), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::Or) => Affix::Infix(Precedence(4), Associativity::Left),

            // TODO: ternary if/else

            ExprIR::Binary(ASTBinaryOp::Range) => Affix::Infix(Precedence(3), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::TypeCast) => Affix::Infix(Precedence(2), Associativity::Left),

            ExprIR::Binary(ASTBinaryOp::Assignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::PlusAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::MinusAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::MultiplyAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::DivideAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::PowerAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::RemainderAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::BitwiseAndAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::BitwiseOrAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::BitwiseXorAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::BitwiseNotAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::BitwiseShiftLeftAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
            ExprIR::Binary(ASTBinaryOp::BitwiseShiftRightAssignment) => Affix::Infix(Precedence(1), Associativity::Left),
        })
    }

    fn primary(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(match input {
            ExprIR::Primary(v) => v,
            ExprIR::Group(inner) => self.parse(&mut inner.into_iter()).unwrap(),
            _ => unreachable!(),
        })
    }

    fn infix(
        &mut self,
        lhs: Self::Output,
        op: Self::Input,
        rhs: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        Ok(ASTValue::BinaryExpr(Box::new(lhs), op.into_binary().unwrap(), Box::new(rhs)))
    }

    fn prefix(
        &mut self,
        op: Self::Input,
        rhs: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        Ok(ASTValue::PrefixExpr(op.into_prefix().unwrap(), Box::new(rhs)))
    }

    fn postfix(
        &mut self,
        lhs: Self::Output,
        op: Self::Input,
    ) -> Result<Self::Output, Self::Error> {
        unreachable!()
    }
}
