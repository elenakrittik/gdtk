pub macro peek_non_blank($iter:expr) {{
    type TokenKind<'a> = $crate::TokenKind<'a>;

    loop {
        if let Some(token) = $iter.peek() {
            match token.kind {
                TokenKind::Blank(_) => {
                    $iter.next();
                }
                _ => break token,
            }
        } else {
            panic!("unexpected EOF");
        }
    }
}}
