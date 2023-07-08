// pub fn identifier<Input>() -> impl Parser<Input, Output = String>
// where
//     Input: Stream<Token = char>,
// {
//     let ident_char: _ = letter().or(cchar('_')).silent();
//     let other_ident_char: _ = alpha_num().or(cchar('_')).silent();
//     let ident_chars: _ = many::<Vec<char>, Input, _>(other_ident_char);

//     (ident_char, ident_chars).map(|(first_char, chars)| {
//         let mut chars_ = vec![first_char];
//         chars_.extend(chars);
//         chars_
//             .into_iter()
//             .map(|c| c.to_string())
//             .collect::<Vec<String>>()
//             .concat()
//     })
// }

// pub fn comment<Input>() -> impl Parser<Input, Output = ASTStatement>
// where
//     Input: Stream<Token = char>,
// {
//     (cchar('#'), many(satisfy(|c| c != '\n'))).map(|(_, chars): (_, Vec<char>)| {
//         ASTStatement::Comment(
//             chars
//                 .into_iter()
//                 .map(|c| c.to_string())
//                 .collect::<Vec<String>>()
//                 .concat(),
//         )
//     })
// }
