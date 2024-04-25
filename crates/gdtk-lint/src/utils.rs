#[macro_export]
macro_rules! lint {
    ($ident:ident) => {
        #[derive(Default)]
        pub struct $ident<'s>(pub Vec<diagnosis::Diagnostic<'s>>);
    };
}
