use crate::token::Token;

macro_rules! test_eq {
    ($input: expr, $($expected: expr),*) => {
        {
            let lexed = $crate::lex($input);
            let mut lexemes = vec![];
            for (token, _) in lexed.0 {
                lexemes.push(token);
            }

            assert_eq!(lexemes, vec![$($expected),*]);
        }
    };
}

macro_rules! test_all_fails {
    ($input: expr) => {{
        let lexed = $crate::lex($input);
        assert!(lexed.0.len() == 0 && lexed.1.len() > 0);
    }};
}

#[test]
fn test_identifier() {
    test_eq!("x", Token::Identifier("x"));
    test_eq!("xyz", Token::Identifier("xyz"));
    test_eq!("XYZ", Token::Identifier("XYZ"));
    test_eq!("X1", Token::Identifier("X1"));
    test_eq!("X1X", Token::Identifier("X1X"));
    test_eq!("X_", Token::Identifier("X_"));
    test_eq!("X_X", Token::Identifier("X_X"));
    test_eq!("_X", Token::Identifier("_X"));
    test_eq!("X1_", Token::Identifier("X1_"));
    test_eq!("X_1", Token::Identifier("X_1"));
    test_eq!("X_1X", Token::Identifier("X_1X"));
    test_eq!("X1_X", Token::Identifier("X1_X"));
    test_eq!("X__X", Token::Identifier("X__X"));
    test_eq!("X_X1", Token::Identifier("X_X1"));
    test_eq!("你", Token::Identifier("你"));
    test_eq!("你好", Token::Identifier("你好"));
    test_eq!("你1", Token::Identifier("你1"));
    test_eq!("你1你", Token::Identifier("你1你"));
    test_eq!("你_", Token::Identifier("你_"));
    test_eq!("你_你", Token::Identifier("你_你"));
    test_eq!("_你", Token::Identifier("_你"));
    test_eq!("你1_", Token::Identifier("你1_"));
    test_eq!("你_1", Token::Identifier("你_1"));
    test_eq!("你_1你", Token::Identifier("你_1你"));
    test_eq!("你1_你", Token::Identifier("你1_你"));
    test_eq!("你__你", Token::Identifier("你__你"));
    test_eq!("你_你1", Token::Identifier("你_你1"));
    test_eq!("п", Token::Identifier("п"));
    test_eq!("привет", Token::Identifier("привет"));
    test_eq!("ПРИВЕТ", Token::Identifier("ПРИВЕТ"));
    test_eq!("П1", Token::Identifier("П1"));
    test_eq!("П1П", Token::Identifier("П1П"));
    test_eq!("П_", Token::Identifier("П_"));
    test_eq!("П_П", Token::Identifier("П_П"));
    test_eq!("_П", Token::Identifier("_П"));
    test_eq!("П1_", Token::Identifier("П1_"));
    test_eq!("П_1", Token::Identifier("П_1"));
    test_eq!("П_1П", Token::Identifier("П_1П"));
    test_eq!("П1_П", Token::Identifier("П1_П"));
    test_eq!("П__П", Token::Identifier("П__П"));
    test_eq!("П_П1", Token::Identifier("П_П1"));
}

#[test]
fn test_integer() {
    test_eq!("0", Token::Integer(0));
    test_eq!("1", Token::Integer(1));
    test_eq!("01", Token::Integer(1));
    test_eq!("001", Token::Integer(1));
    test_eq!("10", Token::Integer(10));
    test_eq!("100", Token::Integer(100));
    test_eq!("101", Token::Integer(101));
}

#[test]
fn test_binaryinteger() {
    test_eq!("0b0", Token::BinaryInteger(0));
    test_eq!("0b1", Token::BinaryInteger(1));
    test_eq!("0b01", Token::BinaryInteger(1));
    test_eq!("0b001", Token::BinaryInteger(1));
    test_eq!("0b10", Token::BinaryInteger(2));
    test_eq!("0b100", Token::BinaryInteger(4));
    test_eq!("0b101", Token::BinaryInteger(5));
}

#[test]
fn test_hexinteger() {
    test_eq!("0x0", Token::HexInteger(0));
    test_eq!("0x1", Token::HexInteger(1));
    test_eq!("0x01", Token::HexInteger(1));
    test_eq!("0x10", Token::HexInteger(16));
    test_eq!("0x123", Token::HexInteger(291));
    test_eq!("0xffffff", Token::HexInteger(16777215));
    test_eq!("0xf62fda", Token::HexInteger(16134106));
    test_eq!("0xf62fdaa", Token::HexInteger(258145706));
}

#[test]
fn test_scientificfloat() {
    test_eq!("1.5e+0", Token::ScientificFloat(1.5));
    test_eq!("1.5e+1", Token::ScientificFloat(15.0));
    test_eq!("1.5e+2", Token::ScientificFloat(150.0));
    test_eq!("1.5e-0", Token::ScientificFloat(1.5));
    test_eq!("1.5e-1", Token::ScientificFloat(0.15));
    test_eq!("1.5e-2", Token::ScientificFloat(0.015));
}

#[test]
fn test_float() {
    test_eq!("0.0", Token::Float(0.0));
    test_eq!("0.1", Token::Float(0.1));
    test_eq!("0.01", Token::Float(0.01));
    test_eq!("1.0", Token::Float(1.0));
    test_eq!("10.0", Token::Float(10.0));
    test_eq!("100.0", Token::Float(100.0));
}

#[test]
fn test_number_underscores() {
    test_eq!("0_1", Token::Integer(1));
    test_eq!("0_0_1", Token::Integer(1));
    test_eq!("1_0", Token::Integer(10));
    test_eq!("1_00", Token::Integer(100));
    test_eq!("10_1", Token::Integer(101));

    test_eq!("0b0_1", Token::BinaryInteger(1));
    test_eq!("0b0_0_1", Token::BinaryInteger(1));
    test_eq!("0b1_0", Token::BinaryInteger(2));
    test_eq!("0b1_00", Token::BinaryInteger(4));
    test_eq!("0b10_1", Token::BinaryInteger(5));

    test_eq!("0x0_1", Token::HexInteger(1));
    test_eq!("0x1_0", Token::HexInteger(16));
    test_eq!("0x1_2_3", Token::HexInteger(291));
    test_eq!("0xff_ff_ff", Token::HexInteger(16777215));
    test_eq!("0xf_62_f_da", Token::HexInteger(16134106));
    test_eq!("0xf6_2f_d_a_a", Token::HexInteger(258145706));

    test_eq!("1.5e+0", Token::ScientificFloat(1.5));
    test_eq!("1_5.0e+1", Token::ScientificFloat(150.0));
    test_eq!("15.0_0e+1", Token::ScientificFloat(150.0));
    test_eq!("1.5e-0_0", Token::ScientificFloat(1.5));
    test_eq!("1.5e-0_1", Token::ScientificFloat(0.15));
    test_eq!("15.0e-0_2", Token::ScientificFloat(0.15));

    test_eq!("0.0_1", Token::Float(0.01));
    test_eq!("1_0.0", Token::Float(10.0));
    test_eq!("1_0_0.0", Token::Float(100.0));
    test_eq!("10.0_0", Token::Float(10.0));
    test_eq!("1_00.0", Token::Float(100.0));
    test_eq!("10_0.0", Token::Float(100.0));
}

#[test]
fn test_string() {
    test_eq!(r#""hello""#, Token::String("hello"));
    test_eq!(r#""你好""#, Token::String("你好"));
    test_eq!(r#""привет""#, Token::String("привет"));
    test_eq!(r#""~!@#$%^&*()_+-=`|""#, Token::String("~!@#$%^&*()_+-=`|"));
    test_eq!(r#""\r\n\f\\""#, Token::String(r"\r\n\f\\")); // TODO
    test_eq!(r#""""#, Token::String(""));
}

#[test]
fn test_stringname() {
    test_eq!(r#"&"hello""#, Token::StringName("hello"));
    test_eq!(r#"&"你好""#, Token::StringName("你好"));
    test_eq!(r#"&"привет""#, Token::StringName("привет"));
    test_eq!(
        r#"&"~!@#$%^&*()_+-=`|""#,
        Token::StringName("~!@#$%^&*()_+-=`|")
    );
    test_eq!(r#"&"\r\n\f\\""#, Token::StringName(r"\r\n\f\\")); // TODO
    test_eq!(r#"&"""#, Token::StringName(""));
}

#[test]
fn test_node() {
    test_eq!(r#"$"Sprite2D""#, Token::Node("Sprite2D"));
    test_eq!(r#"$"Player/Sprite2D""#, Token::Node("Player/Sprite2D"));
    test_eq!(r#"$"Player/привет""#, Token::Node("Player/привет"));
}

#[test]
fn test_uniquenode() {
    test_eq!(
        r#"%"PlayerAnimation""#,
        Token::UniqueNode("PlayerAnimation")
    );
}

#[test]
fn test_nodepath() {
    test_eq!(r#"^"Sprite2D""#, Token::NodePath("Sprite2D"));
    test_eq!(r#"^"Player/Sprite2D""#, Token::NodePath("Player/Sprite2D"));
    test_eq!(r#"^"Player/привет""#, Token::NodePath("Player/привет"));
}

#[test]
fn test_boolean() {
    test_eq!("true", Token::Boolean(true));
    test_eq!("false", Token::Boolean(false));
}

#[test]
fn test_null() {
    test_eq!("null", Token::Null);
}

#[test]
fn test_comparison() {
    test_eq!("<", Token::Less);
    test_eq!("<=", Token::LessEqual);
    test_eq!(">", Token::Greater);
    test_eq!(">=", Token::GreaterEqual);
    test_eq!("==", Token::Equal);
    test_eq!("!=", Token::NotEqual);
}

#[test]
fn test_logical() {
    test_eq!("and", Token::And);
    test_eq!("or", Token::Or);
    test_eq!("not", Token::Not);
    test_eq!("&&", Token::SymbolizedAnd);
    test_eq!("||", Token::SymbolizedOr);
    test_eq!("!", Token::SymbolizedNot);
}

#[test]
fn test_bitwise() {
    test_eq!("&", Token::BitwiseAnd);
    test_eq!("|", Token::BitwiseOr);
    test_eq!("~", Token::BitwiseNot);
    test_eq!("^", Token::BitwiseXor);
    test_eq!("<<", Token::BitwiseShiftLeft);
    test_eq!(">>", Token::BitwiseShiftRight);
}

#[test]
fn test_math() {
    test_eq!("+", Token::Plus);
    test_eq!("-", Token::Minus);
    test_eq!("*", Token::Multiply);
    test_eq!("**", Token::Power);
    test_eq!("/", Token::Divide);
    test_eq!("%", Token::Remainder);
}

#[test]
fn test_assignment() {
    test_eq!("=", Token::Assignment);
    test_eq!("+=", Token::PlusAssignment);
    test_eq!("-=", Token::MinusAssignment);
    test_eq!("*=", Token::MultiplyAssignment);
    test_eq!("**=", Token::PowerAssignment);
    test_eq!("/=", Token::DivideAssignment);
    test_eq!("%=", Token::RemainderAssignment);
    test_eq!("&=", Token::BitwiseAndAssignment);
    test_eq!("|=", Token::BitwiseOrAssignment);
    test_eq!("~=", Token::BitwiseNotAssignment);
    test_eq!("^=", Token::BitwiseXorAssignment);
    test_eq!("<<=", Token::BitwiseShiftLeftAssignment);
    test_eq!(">>=", Token::BitwiseShiftRightAssignment);
}

#[test]
fn test_control_flow() {
    test_eq!("if", Token::If);
    test_eq!("elif", Token::Elif);
    test_eq!("else", Token::Else);
    test_eq!("for", Token::For);
    test_eq!("while", Token::While);
    test_eq!("break", Token::Break);
    test_eq!("continue", Token::Continue);
    test_eq!("pass", Token::Pass);
    test_eq!("return", Token::Return);
    test_eq!("match", Token::Match);
}

#[test]
fn test_keywords() {
    test_eq!("as", Token::As);
    test_eq!("assert", Token::Assert);
    test_eq!("await", Token::Await);
    test_eq!("breakpoint", Token::Breakpoint);
    test_eq!("class", Token::Class);
    test_eq!("class_name", Token::ClassName);
    test_eq!("const", Token::Const);
    test_eq!("enum", Token::Enum);
    test_eq!("extends", Token::Extends);
    test_eq!("func", Token::Func);
    test_eq!("in", Token::In);
    test_eq!("is", Token::Is);
    test_eq!("signal", Token::Signal);
    test_eq!("static", Token::Static);
    test_eq!("var", Token::Var);
}

#[test]
fn test_punctuation() {
    test_eq!("@", Token::Annotation);
    test_eq!("(", Token::OpeningParenthesis);
    test_eq!(")", Token::ClosingParenthesis);
    test_eq!("[", Token::OpeningBracket);
    test_eq!("]", Token::ClosingBracket);
    test_eq!("{", Token::OpeningBrace);
    test_eq!("}", Token::ClosingBrace);
    test_eq!(",", Token::Comma);
    test_eq!(";", Token::Semicolon);
    test_eq!(".", Token::Period);
    test_eq!("..", Token::Range);
    test_eq!(":", Token::Colon);
    test_eq!("$", Token::Dollar);
    test_eq!("->", Token::Arrow);
}

#[test]
fn test_whitespace() {
    test_eq!("\n", Token::Newline);
    test_eq!("\r\n", Token::Newline);
    test_eq!(" ", Token::Blank(" "));
    test_eq!("  ", Token::Blank("  "));
    test_eq!("\t", Token::Blank("\t"));
    test_eq!("\t\t", Token::Blank("\t\t"));
    test_eq!(" \t", Token::Blank(" \t"));
    test_eq!("\t ", Token::Blank("\t "));
    test_eq!("\t \t", Token::Blank("\t \t"));
    test_eq!(" \t ", Token::Blank(" \t "));
}

#[test]
fn test_comment() {
    test_eq!("#hello", Token::Comment("hello"));
    test_eq!("# hello", Token::Comment(" hello"));
    test_eq!("#привет", Token::Comment("привет"));
    test_eq!("# привет", Token::Comment(" привет"));
    test_eq!("#你好", Token::Comment("你好"));
    test_eq!("# 你好", Token::Comment(" 你好"));
    test_eq!("##hello", Token::Comment("#hello"));
    test_eq!("## hello", Token::Comment("# hello"));
    test_eq!("#hello#", Token::Comment("hello#"));
    test_eq!("# hello#", Token::Comment(" hello#"));
    test_eq!("#hello #", Token::Comment("hello #"));
    test_eq!("# hello #", Token::Comment(" hello #"));
    test_eq!("# hello\n", Token::Comment(" hello"), Token::Newline);
}

#[test]
fn test_radt() {
    test_eq!("namespace", Token::Namespace);
    test_eq!("trait", Token::Trait);
    test_eq!("yield", Token::Yield);
}

#[test]
fn test_invalid() {
    test_all_fails!("`");
    test_all_fails!("\0");
    test_all_fails!("\r");
}
