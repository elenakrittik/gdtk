#![feature(decl_macro)]

pub macro unwrap_enum($val:ident, $arm:pat, $inner:ident) {
    match $val {
        $arm => $inner,
        _ => unreachable!(),
    }
}
