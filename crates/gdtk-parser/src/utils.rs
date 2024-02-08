// TODO: refactor some stuff to utilize new option to .peek()

pub macro any_assignment($enm:ident) {
    $enm::Assignment
    | $enm::PlusAssignment
    | $enm::MinusAssignment
    | $enm::MultiplyAssignment
    | $enm::PowerAssignment
    | $enm::DivideAssignment
    | $enm::RemainderAssignment
    | $enm::BitwiseAndAssignment
    | $enm::BitwiseOrAssignment
    | $enm::BitwiseNotAssignment
    | $enm::BitwiseXorAssignment
    | $enm::BitwiseShiftLeftAssignment
    | $enm::BitwiseShiftRightAssignment
}

pub macro expect($iter:expr, $variant:pat, $ret:expr) {{
    type Token<'a> = ::gdtk_lexer::Token<'a>;

    match $iter.next() {
        Some(Token { kind: $variant, .. }) => $ret,
        other => panic!("expected {}, found {other:?}", stringify!($variant)),
    }
}}

pub macro expect_blank_prefixed($iter:expr, $variant:pat, $ret:expr) {{
    type TokenKind<'a> = ::gdtk_lexer::TokenKind<'a>;

    loop {
        if let Some(token) = $iter.next() {
            match token.kind {
                TokenKind::Blank(_) => (),
                $variant => break $ret,
                _ => panic!("expected {}, found {token:?}", stringify!($variant)),
            }
        } else {
            panic!("unexpected EOF");
        }
    }
}}

pub macro peek_non_blank($iter:expr) {{
    type TokenKind<'a> = ::gdtk_lexer::TokenKind<'a>;

    loop {
        if let Some(token) = $iter.peek() {
            match token.kind {
                TokenKind::Blank(_) => { $iter.next(); },
                _ => break token,
            }
        } else {
            panic!("unexpected EOF");
        }
    }
}}

pub macro next_non_blank($iter:expr) {{
    type TokenKind<'a> = ::gdtk_lexer::TokenKind<'a>;

    loop {
        if let Some(token) = $iter.next() {
            match token.kind {
                TokenKind::Blank(_) => (),
                _ => break token,
            }
        } else {
            panic!("unexpected EOF");
        }
    }
}}

pub macro collect_args($iter:expr, $opening:pat, $closing:pat) {{
    $crate::utils::expect!($iter, $opening, ());
    $crate::utils::collect_args_raw!($iter, $closing)
}}

pub macro collect_args_raw($iter:expr, $closing:pat) {{
    type TokenKind<'a> = ::gdtk_lexer::TokenKind<'a>;

    let mut args = vec![];
    let mut expect_comma = false;

    while let Some(token) = $iter.next() {
        match &token.kind {
            &TokenKind::Comma => {
                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            }
            &TokenKind::Blank(_) => (),
            &$closing => break,
            other => {
                if expect_comma {
                    panic!("expected comma, got {other:?}");
                }
                args.push($crate::values::parse_value($iter, Some(token)));
                expect_comma = true;
            }
        }
    }

    args
}}

/// Parse identifier: type = default
pub macro parse_idtydef($iter:expr, $($endpat:pat => $endcode:expr,)*) {{
    type Token<'a> = ::gdtk_lexer::Token<'a>;
    type TokenKind<'a> = ::gdtk_lexer::TokenKind<'a>;

    let identifier = $crate::utils::expect_blank_prefixed!($iter, TokenKind::Identifier(s), s);

    let mut infer_type = false;
    let mut typehint = None;
    let mut value = None;

    // a colon, an assignment or a newline
    match $crate::utils::next_non_blank!($iter) {
        Token { kind: TokenKind::Colon, .. } => {
            // colon can be followed by an identifier (typehint) or an assignment (means the type should be inferred)
            match $crate::utils::next_non_blank!($iter) {
                Token { kind: TokenKind::Identifier(s), .. } => {
                    // we got the typehint
                    typehint = Some(s);

                    // typehint can be followed by an assignment or a newline
                    match $crate::utils::next_non_blank!($iter) {
                        // found assignment, then there must be a value
                        Token { kind: TokenKind::Assignment, .. } => value = Some($crate::values::parse_value($iter, None)),
                        // no value
                        $(Token { kind: $endpat, .. } => $endcode,)*
                        other => panic!("unexpected {other:?}, expected assignment or newline"),
                    }
                },
                Token { kind: TokenKind::Assignment, .. } => {
                    infer_type = true;
                    value = Some($crate::values::parse_value($iter, None));
                },
                other => panic!("unexpected {other:?}, expected assignment or newline"),
            }
        },
        Token { kind: TokenKind::Assignment, .. } => value = Some($crate::values::parse_value($iter, None)),
        $(Token { kind: $endpat, .. } => $endcode,)*
        other => panic!("unexpected {other:?}, expected colon, assignment or newline"),
    }

    (identifier, infer_type, typehint, value)
}}
