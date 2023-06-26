use combine::{
    choice, many, many1,
    parser::char::{char as cchar, digit},
    Parser, Stream,
};

use super::helpers::safe_end;
use crate::ast::ASTValue;

pub fn value<Input>() -> impl combine::Parser<Input, Output = ASTValue>
where
    Input: combine::Stream<Token = char>,
{
    choice((int(),))
}

pub fn int<Input>() -> impl Parser<Input, Output = ASTValue>
where
    Input: Stream<Token = char>,
{
    let negate = many(cchar('-')).map(|chars: Vec<char>| {
        if chars.len() % 2 == 0 {
            // according to fundamental math laws, --2 is the same as 2
            None
        } else {
            // and likewise ---2 is the same as -2
            Some("-")
        } // poor code monkeys like me don't understand why but hopefully you do :^
    });

    (
        negate,
        many1(digit()).map(|chars: Vec<char>| {
            chars
                .iter()
                .collect::<String>()
                .parse::<i64>()
                .expect("valid integer string")
        }),
        safe_end(),
    )
        .map(|(minus, mut num, _)| {
            if minus.is_some() {
                num = -num;
            }

            ASTValue::Int(num)
        })
}

#[cfg(test)]
mod test {
    use combine::Parser;

    use crate::{ast::ASTValue, parser::values::int};

    #[test]
    fn test_int() {
        assert_eq!(int().parse("0").unwrap().0, ASTValue::Int(0));
        assert_eq!(int().parse("1").unwrap().0, ASTValue::Int(1));
        assert_eq!(int().parse("-1").unwrap().0, ASTValue::Int(-1));
        assert_eq!(int().parse("-0").unwrap().0, ASTValue::Int(0));
        assert_eq!(int().parse("01").unwrap().0, ASTValue::Int(1)); // yes, apparently this is valid gdscript
        assert_eq!(int().parse("-01").unwrap().0, ASTValue::Int(-1));
        assert_eq!(int().parse("--1").unwrap().0, ASTValue::Int(1));
        assert_eq!(int().parse("---1").unwrap().0, ASTValue::Int(-1));
        assert!(int().parse("").is_err());
        assert!(int().parse("-").is_err());
        assert!(int().parse("a").is_err());
        assert!(int().parse(" ").is_err());
        assert!(int().parse("1-").is_err()); // todo
        assert!(int().parse(" 1").is_err());
        assert!(int().parse("- 1").is_err());
        println!("{:?}", int().parse("-1 "));
        assert!(int().parse("-1 ").is_err());
        assert!(int().parse("0x").is_err()); // todo
    }
}
