use sparsec::Sparsec;

use crate::ast::ASTValue;

pub fn value(parser: &Sparsec) -> anyhow::Result<ASTValue> {
    Ok(int(parser)?)
    // Ok(ASTValue::Float(15.01))
}

pub fn int(parser: &Sparsec) -> anyhow::Result<ASTValue> {
    Ok(ASTValue::Int(integer(parser)?))
}

// pub fn float(parser: &mut Sparsec) -> anyhow::Result<ASTValue> {
//     let integer_ = integer(parser, true)?;

//     parser.read_one_exact('.')?;

//     let fraction = integer(parser, false)?;

//     let f = format!("{}.{}", integer_, fraction).parse()?;

//     Ok(ASTValue::Float(f))
// }

fn integer(parser: &mut Sparsec, /*, allow_minus: bool */) -> anyhow::Result<i64, anyhow::Error> {
    if true
    /* allow_minus */
    {
        let neg = parser
            .by_ref()
            .take_while(|c| *c == '-')
            .collect::<Vec<char>>()
            .len()
            % 2
            == 1;

        let mut val: i64 = parser
            .by_ref()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()?;

        if neg {
            val = -val
        };

        Ok(val)
    } else {
        Ok(parser
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()?)
    }
}

// pub fn string(parser: &mut Sparsec) -> anyhow::Result<ASTValue> {
//     parser.read_one_exact('"')?;
//     let content = parser.read_until('\"')?;
//     parser.read_one_exact('"')?;

//     Ok(ASTValue::String(content.iter().collect()))
// }

#[cfg(test)]
mod test {
    use crate::{ast::ASTValue, parser::values::int};

    #[test]
    fn test_int() {
        sparsec::test_eq!(int, "0", ASTValue::Int(0));
        sparsec::test_eq!(int, "1", ASTValue::Int(1));
        sparsec::test_eq!(int, "-1", ASTValue::Int(-1));
        sparsec::test_eq!(int, "-0", ASTValue::Int(0));
        sparsec::test_eq!(int, "01", ASTValue::Int(1)); // apparently this is valid
        sparsec::test_eq!(int, "-01", ASTValue::Int(-1));
        sparsec::test_eq!(int, "--1", ASTValue::Int(1));
        sparsec::test_eq!(int, "---1", ASTValue::Int(-1));
        sparsec::test_fails!(int, "");
        sparsec::test_fails!(int, "-");
        sparsec::test_fails!(int, "a");
        sparsec::test_fails!(int, " ");
        // sparsec::test_fails!(int, "1-"); // todo // never:tm:
        sparsec::test_fails!(int, " 1");
        sparsec::test_fails!(int, "- 1");
        // sparsec::test_fails!(int, "-1 "); // todo // never:tm:
        // sparsec::test_fails!(int, "0x"); // todo // never:tm:
    }
}
