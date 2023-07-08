// pub fn newline_<R: Read>(parser: Sparsec<R>) -> () {}

// pub fn newlines<Input>() -> impl Parser<Input>
// where
//     Input: Stream<Token = char>,
// {
//     many(newline()).map(|_: Vec<char>| ())
// }

// pub fn simple_statement<Input>(
//     keyword: &'static str,
// ) -> impl Parser<Input, Output = (&str, (), String, ())>
// where
//     Input: Stream<Token = char>,
// {
//     (
//         string(keyword),
//         spaces().silent(),
//         identifier(),
//         spaces().silent(),
//     )
// }

// pub fn safe_end<Input>() -> impl combine::Parser<Input, Output = ()>
// where
//     Input: combine::Stream<Token = char>,
// {
//     attempt(choice((
//         string(" #"),
//         string(" \n"),
//         string(" ;"),
//         (string(" "), eof()).map(|_| " "),
//         string("#"),
//         string("\n"),
//         string(";"),
//     )))
//     .map(|_| ())
//     .or(eof())
// }
