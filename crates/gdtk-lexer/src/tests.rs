use crate::token::TokenKind;

macro_rules! test_eq {
    ($input: expr, $($expected: expr),*) => {
        {
            let lexed = $crate::lex($input);
            let mut lexemes = vec![];

            for token in lexed.0 {
                lexemes.push(token.kind);
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
    test_eq!("x", TokenKind::Identifier("x"));
    test_eq!("xyz", TokenKind::Identifier("xyz"));
    test_eq!("XYZ", TokenKind::Identifier("XYZ"));
    test_eq!("X1", TokenKind::Identifier("X1"));
    test_eq!("X1X", TokenKind::Identifier("X1X"));
    test_eq!("X_", TokenKind::Identifier("X_"));
    test_eq!("X_X", TokenKind::Identifier("X_X"));
    test_eq!("_X", TokenKind::Identifier("_X"));
    test_eq!("X1_", TokenKind::Identifier("X1_"));
    test_eq!("X_1", TokenKind::Identifier("X_1"));
    test_eq!("X_1X", TokenKind::Identifier("X_1X"));
    test_eq!("X1_X", TokenKind::Identifier("X1_X"));
    test_eq!("X__X", TokenKind::Identifier("X__X"));
    test_eq!("X_X1", TokenKind::Identifier("X_X1"));
    test_eq!("你", TokenKind::Identifier("你"));
    test_eq!("你好", TokenKind::Identifier("你好"));
    test_eq!("你1", TokenKind::Identifier("你1"));
    test_eq!("你1你", TokenKind::Identifier("你1你"));
    test_eq!("你_", TokenKind::Identifier("你_"));
    test_eq!("你_你", TokenKind::Identifier("你_你"));
    test_eq!("_你", TokenKind::Identifier("_你"));
    test_eq!("你1_", TokenKind::Identifier("你1_"));
    test_eq!("你_1", TokenKind::Identifier("你_1"));
    test_eq!("你_1你", TokenKind::Identifier("你_1你"));
    test_eq!("你1_你", TokenKind::Identifier("你1_你"));
    test_eq!("你__你", TokenKind::Identifier("你__你"));
    test_eq!("你_你1", TokenKind::Identifier("你_你1"));
    test_eq!("п", TokenKind::Identifier("п"));
    test_eq!("привет", TokenKind::Identifier("привет"));
    test_eq!("ПРИВЕТ", TokenKind::Identifier("ПРИВЕТ"));
    test_eq!("П1", TokenKind::Identifier("П1"));
    test_eq!("П1П", TokenKind::Identifier("П1П"));
    test_eq!("П_", TokenKind::Identifier("П_"));
    test_eq!("П_П", TokenKind::Identifier("П_П"));
    test_eq!("_П", TokenKind::Identifier("_П"));
    test_eq!("П1_", TokenKind::Identifier("П1_"));
    test_eq!("П_1", TokenKind::Identifier("П_1"));
    test_eq!("П_1П", TokenKind::Identifier("П_1П"));
    test_eq!("П1_П", TokenKind::Identifier("П1_П"));
    test_eq!("П__П", TokenKind::Identifier("П__П"));
    test_eq!("П_П1", TokenKind::Identifier("П_П1"));
}

#[test]
fn test_integer() {
    test_eq!("0", TokenKind::Integer(0));
    test_eq!("1", TokenKind::Integer(1));
    test_eq!("01", TokenKind::Integer(1));
    test_eq!("001", TokenKind::Integer(1));
    test_eq!("10", TokenKind::Integer(10));
    test_eq!("100", TokenKind::Integer(100));
    test_eq!("101", TokenKind::Integer(101));
}

#[test]
fn test_binaryinteger() {
    test_eq!("0b0", TokenKind::BinaryInteger(0));
    test_eq!("0b1", TokenKind::BinaryInteger(1));
    test_eq!("0b01", TokenKind::BinaryInteger(1));
    test_eq!("0b001", TokenKind::BinaryInteger(1));
    test_eq!("0b10", TokenKind::BinaryInteger(2));
    test_eq!("0b100", TokenKind::BinaryInteger(4));
    test_eq!("0b101", TokenKind::BinaryInteger(5));
}

#[test]
fn test_hexinteger() {
    test_eq!("0x0", TokenKind::HexInteger(0));
    test_eq!("0x1", TokenKind::HexInteger(1));
    test_eq!("0x01", TokenKind::HexInteger(1));
    test_eq!("0x10", TokenKind::HexInteger(16));
    test_eq!("0x123", TokenKind::HexInteger(291));
    test_eq!("0xffffff", TokenKind::HexInteger(16777215));
    test_eq!("0xf62fda", TokenKind::HexInteger(16134106));
    test_eq!("0xf62fdaa", TokenKind::HexInteger(258145706));
}

#[test]
fn test_scientificfloat() {
    test_eq!("1.5e+0", TokenKind::ScientificFloat(1.5));
    test_eq!("1.5e+1", TokenKind::ScientificFloat(15.0));
    test_eq!("1.5e+2", TokenKind::ScientificFloat(150.0));
    test_eq!("1.5e-0", TokenKind::ScientificFloat(1.5));
    test_eq!("1.5e-1", TokenKind::ScientificFloat(0.15));
    test_eq!("1.5e-2", TokenKind::ScientificFloat(0.015));
}

#[test]
fn test_float() {
    test_eq!("0.0", TokenKind::Float(0.0));
    test_eq!("0.1", TokenKind::Float(0.1));
    test_eq!("0.01", TokenKind::Float(0.01));
    test_eq!("1.0", TokenKind::Float(1.0));
    test_eq!("10.0", TokenKind::Float(10.0));
    test_eq!("100.0", TokenKind::Float(100.0));
}

#[test]
fn test_number_underscores() {
    test_eq!("0_1", TokenKind::Integer(1));
    test_eq!("0_0_1", TokenKind::Integer(1));
    test_eq!("1_0", TokenKind::Integer(10));
    test_eq!("1_00", TokenKind::Integer(100));
    test_eq!("10_1", TokenKind::Integer(101));

    test_eq!("0b0_1", TokenKind::BinaryInteger(1));
    test_eq!("0b0_0_1", TokenKind::BinaryInteger(1));
    test_eq!("0b1_0", TokenKind::BinaryInteger(2));
    test_eq!("0b1_00", TokenKind::BinaryInteger(4));
    test_eq!("0b10_1", TokenKind::BinaryInteger(5));

    test_eq!("0x0_1", TokenKind::HexInteger(1));
    test_eq!("0x1_0", TokenKind::HexInteger(16));
    test_eq!("0x1_2_3", TokenKind::HexInteger(291));
    test_eq!("0xff_ff_ff", TokenKind::HexInteger(16777215));
    test_eq!("0xf_62_f_da", TokenKind::HexInteger(16134106));
    test_eq!("0xf6_2f_d_a_a", TokenKind::HexInteger(258145706));

    test_eq!("1.5e+0", TokenKind::ScientificFloat(1.5));
    test_eq!("1_5.0e+1", TokenKind::ScientificFloat(150.0));
    test_eq!("15.0_0e+1", TokenKind::ScientificFloat(150.0));
    test_eq!("1.5e-0_0", TokenKind::ScientificFloat(1.5));
    test_eq!("1.5e-0_1", TokenKind::ScientificFloat(0.15));
    test_eq!("15.0e-0_2", TokenKind::ScientificFloat(0.15));

    test_eq!("0.0_1", TokenKind::Float(0.01));
    test_eq!("1_0.0", TokenKind::Float(10.0));
    test_eq!("1_0_0.0", TokenKind::Float(100.0));
    test_eq!("10.0_0", TokenKind::Float(10.0));
    test_eq!("1_00.0", TokenKind::Float(100.0));
    test_eq!("10_0.0", TokenKind::Float(100.0));
}

#[test]
fn test_string() {
    test_eq!(r#""hello""#, TokenKind::String("hello"));
    test_eq!(r#""你好""#, TokenKind::String("你好"));
    test_eq!(r#""привет""#, TokenKind::String("привет"));
    test_eq!(
        r#""~!@#$%^&*()_+-=`|""#,
        TokenKind::String("~!@#$%^&*()_+-=`|")
    );
    test_eq!(r#""\r\n\f\\""#, TokenKind::String(r"\r\n\f\\")); // TODO
    test_eq!(r#""""#, TokenKind::String(""));
}

#[test]
fn test_stringname() {
    test_eq!(r#"&"hello""#, TokenKind::StringName("hello"));
    test_eq!(r#"&"你好""#, TokenKind::StringName("你好"));
    test_eq!(r#"&"привет""#, TokenKind::StringName("привет"));
    test_eq!(
        r#"&"~!@#$%^&*()_+-=`|""#,
        TokenKind::StringName("~!@#$%^&*()_+-=`|")
    );
    test_eq!(r#"&"\r\n\f\\""#, TokenKind::StringName(r"\r\n\f\\")); // TODO
    test_eq!(r#"&"""#, TokenKind::StringName(""));
}

#[test]
fn test_node() {
    test_eq!(r#"$"Sprite2D""#, TokenKind::Node("Sprite2D"));
    test_eq!(r#"$"Player/Sprite2D""#, TokenKind::Node("Player/Sprite2D"));
    test_eq!(r#"$"Player/привет""#, TokenKind::Node("Player/привет"));
}

#[test]
fn test_uniquenode() {
    test_eq!(
        r#"%"PlayerAnimation""#,
        TokenKind::UniqueNode("PlayerAnimation")
    );
}

#[test]
fn test_nodepath() {
    test_eq!(r#"^"Sprite2D""#, TokenKind::NodePath("Sprite2D"));
    test_eq!(
        r#"^"Player/Sprite2D""#,
        TokenKind::NodePath("Player/Sprite2D")
    );
    test_eq!(r#"^"Player/привет""#, TokenKind::NodePath("Player/привет"));
}

#[test]
fn test_boolean() {
    test_eq!("true", TokenKind::Boolean(true));
    test_eq!("false", TokenKind::Boolean(false));
}

#[test]
fn test_null() {
    test_eq!("null", TokenKind::Null);
}

#[test]
fn test_comparison() {
    test_eq!("<", TokenKind::Less);
    test_eq!("<=", TokenKind::LessOrEqual);
    test_eq!(">", TokenKind::Greater);
    test_eq!(">=", TokenKind::GreaterOrEqual);
    test_eq!("==", TokenKind::Equal);
    test_eq!("!=", TokenKind::NotEqual);
}

#[test]
fn test_logical() {
    test_eq!("and", TokenKind::And);
    test_eq!("or", TokenKind::Or);
    test_eq!("not", TokenKind::Not);
    test_eq!("&&", TokenKind::SymbolizedAnd);
    test_eq!("||", TokenKind::SymbolizedOr);
    test_eq!("!", TokenKind::SymbolizedNot);
}

#[test]
fn test_bitwise() {
    test_eq!("&", TokenKind::BitwiseAnd);
    test_eq!("|", TokenKind::BitwiseOr);
    test_eq!("~", TokenKind::BitwiseNot);
    test_eq!("^", TokenKind::BitwiseXor);
    test_eq!("<<", TokenKind::BitwiseShiftLeft);
    test_eq!(">>", TokenKind::BitwiseShiftRight);
}

#[test]
fn test_math() {
    test_eq!("+", TokenKind::Plus);
    test_eq!("-", TokenKind::Minus);
    test_eq!("*", TokenKind::Multiply);
    test_eq!("**", TokenKind::Power);
    test_eq!("/", TokenKind::Divide);
    test_eq!("%", TokenKind::Remainder);
}

#[test]
fn test_assignment() {
    test_eq!("=", TokenKind::Assignment);
    test_eq!("+=", TokenKind::PlusAssignment);
    test_eq!("-=", TokenKind::MinusAssignment);
    test_eq!("*=", TokenKind::MultiplyAssignment);
    test_eq!("**=", TokenKind::PowerAssignment);
    test_eq!("/=", TokenKind::DivideAssignment);
    test_eq!("%=", TokenKind::RemainderAssignment);
    test_eq!("&=", TokenKind::BitwiseAndAssignment);
    test_eq!("|=", TokenKind::BitwiseOrAssignment);
    test_eq!("~=", TokenKind::BitwiseNotAssignment);
    test_eq!("^=", TokenKind::BitwiseXorAssignment);
    test_eq!("<<=", TokenKind::BitwiseShiftLeftAssignment);
    test_eq!(">>=", TokenKind::BitwiseShiftRightAssignment);
}

#[test]
fn test_control_flow() {
    test_eq!("if", TokenKind::If);
    test_eq!("elif", TokenKind::Elif);
    test_eq!("else", TokenKind::Else);
    test_eq!("for", TokenKind::For);
    test_eq!("while", TokenKind::While);
    test_eq!("break", TokenKind::Break);
    test_eq!("continue", TokenKind::Continue);
    test_eq!("pass", TokenKind::Pass);
    test_eq!("return", TokenKind::Return);
    test_eq!("match", TokenKind::Match);
}

#[test]
fn test_keywords() {
    test_eq!("as", TokenKind::As);
    test_eq!("assert", TokenKind::Assert);
    test_eq!("await", TokenKind::Await);
    test_eq!("breakpoint", TokenKind::Breakpoint);
    test_eq!("class", TokenKind::Class);
    test_eq!("class_name", TokenKind::ClassName);
    test_eq!("const", TokenKind::Const);
    test_eq!("enum", TokenKind::Enum);
    test_eq!("extends", TokenKind::Extends);
    test_eq!("func", TokenKind::Func);
    test_eq!("in", TokenKind::In);
    test_eq!("is", TokenKind::Is);
    test_eq!("signal", TokenKind::Signal);
    test_eq!("static", TokenKind::Static);
    test_eq!("var", TokenKind::Var);
}

#[test]
fn test_punctuation() {
    test_eq!("@", TokenKind::Annotation);
    test_eq!("(", TokenKind::OpeningParenthesis);
    test_eq!(")", TokenKind::ClosingParenthesis);
    test_eq!("[", TokenKind::OpeningBracket);
    test_eq!("]", TokenKind::ClosingBracket);
    test_eq!("{", TokenKind::OpeningBrace);
    test_eq!("}", TokenKind::ClosingBrace);
    test_eq!(",", TokenKind::Comma);
    test_eq!(";", TokenKind::Semicolon);
    test_eq!(".", TokenKind::Period);
    test_eq!("..", TokenKind::Range);
    test_eq!(":", TokenKind::Colon);
    test_eq!("$", TokenKind::Dollar);
    test_eq!("->", TokenKind::Arrow);
}

#[test]
fn test_radt() {
    test_eq!("namespace", TokenKind::Namespace);
    test_eq!("trait", TokenKind::Trait);
    test_eq!("yield", TokenKind::Yield);
}

#[test]
fn test_invalid() {
    test_all_fails!("`");
    test_all_fails!("\0");
    test_all_fails!("\r");
}
