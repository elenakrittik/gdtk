/// Helper macro for use in tests.
///
/// ```
/// fn int(parser: &mut sparsec::Sparsec) -> Result<i64, ()> {
///     let raw = parser.read_while(|c| c.is_ascii_digit()).map_err(|_| ())?;
///     let num = raw.parse().map_err(|_| ())?;
///     Ok(num)
/// }
///
/// sparsec::test_eq!(int, "10", 10);
/// ```
pub macro test_eq($func: ident, $input: expr, $value: expr) {{
    $crate::from_string!(parser, $input);
    assert_eq!($func(&mut parser).unwrap(), $value);
}}

/// Helper macro for use in tests.
///
/// ```
/// fn int(parser: &mut sparsec::Sparsec) -> Result<i64, ()> {
///     let raw = parser.read_while(|c| c.is_ascii_digit()).map_err(|_| ())?;
///     let num = raw.parse().map_err(|_| ())?;
///     Ok(num)
/// }
///
/// sparsec::test_fails!(int, "not an integer");
/// ```
pub macro test_fails($func: ident, $input: expr) {{
    $crate::from_string!(parser, $input);
    assert!($func(&mut parser).is_err());
}}
