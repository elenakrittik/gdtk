pub macro expect($iter:expr, $variant:pat, $ret:expr) {
    match $iter.next() {
        Some($variant) => $ret,
        other => panic!("expected {}, found {other:?}", stringify!($variant)),
    }
}

pub macro expect_blank_prefixed($iter:expr, $variant:pat, $ret:expr) {{
    type Token<'a> = ::gdtk_lexer::Token<'a>;

    loop {
        if let Some(token) = $iter.next() {
            match token {
                Token::Blank(_) => (),
                $variant => break $ret,
                other => panic!("expected {}, found {other:?}", stringify!($variant)),
            }
        } else {
            panic!("unexpected EOF");
        }
    }
}}

pub macro next_non_blank($iter:expr) {{
    type Token<'a> = ::gdtk_lexer::Token<'a>;

    loop {
        if let Some(token) = $iter.next() {
            match token {
                Token::Blank(_) => (),
                other => break other,
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
    type Token<'a> = ::gdtk_lexer::Token<'a>;

    let mut args = vec![];
    let mut expect_comma = false;

    while let Some(token) = $iter.next() {
        match token {
            Token::Comma => {
                if !expect_comma {
                    panic!("unexpected comma, expected a value");
                }
                expect_comma = false;
            },
            Token::Blank(_) => (),
            $closing => break,
            other => {
                if expect_comma {
                    panic!("expected comma, got {other:?}");
                }
                args.push($crate::values::parse_value($iter, Some(other)));
                expect_comma = true;
            }
        }
    }

    args
}}

pub macro parse_idtydef($iter:expr, $($endpat:pat => $endcode:expr,)*) {{
    type Token<'a> = ::gdtk_lexer::Token<'a>;

    let identifier = $crate::utils::expect_blank_prefixed!($iter, Token::Identifier(s), s);

    let mut infer_type = false;
    let mut typehint = None;
    let mut value = None;

    // a colon, an assignment or a newline
    match $crate::utils::next_non_blank!($iter) {
        Token::Colon => {
            // colon can be followed by an identifier (typehint) or an assignment (means the type should be inferred)
            match $crate::utils::next_non_blank!($iter) {
                Token::Identifier(s) => {
                    // we got the typehint
                    typehint = Some(s);

                    // typehint can be followed by an assignment or a newline
                    match $crate::utils::next_non_blank!($iter) {
                        // found assignment, then there must be a value
                        Token::Assignment => value = Some($crate::values::parse_value($iter, None)),
                        // no value
                        $($endpat => $endcode,)*
                        other => panic!("unexpected {other:?}, expected assignment or newline"),
                    }
                },
                Token::Assignment => {
                    infer_type = true;
                    value = Some($crate::values::parse_value($iter, None));
                },
                other => panic!("unexpected {other:?}, expected assignment or newline"),
            }
        },
        Token::Assignment => value = Some($crate::values::parse_value($iter, None)),
        $($endpat => $endcode,)*
        other => panic!("unexpected {other:?}, expected colon, assignment or newline"),
    }

    (identifier, infer_type, typehint, value)
}}
