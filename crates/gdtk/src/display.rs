//! Utility functions for consistent output displaying.

// use ariadne::{ColorGenerator, Source};
// use owo_colors::OwoColorize; // TODO

/// Prints an error using format_error and eprintln
#[inline]
pub fn print_error(text: String) {
    eprintln!("{}", format_error(text));
}

/// Format error text.
#[inline]
pub fn format_error(text: String) -> String {
    format!("{} {}", error("error:"), text)
}

/// Colorize text with error color.
#[inline]
pub fn error(text: &str) -> &str {
    text // TODO
}

/// Colorize text with action (e.g., a command or a "Formatting" status message) color.
#[inline]
pub fn action(text: &str) -> &str {
    text // TODO
}

/// Colorize text with thing (e.g., a path or any other "value") color.
#[inline]
pub fn path(text: &String) -> &String {
    text
}

// pub fn display_ariadne_traceback(
//     input_name: &String,
//     input: &str,
//     e: Errors<char, &str, PointerOffset<str>>,
// ) {
//     let err = e.map_position(|p| p.translate_position(input));

// let mut unexpected = String::new();
// let mut expected = String::new();

//   let mut colors = ColorGenerator::new();
//    let expected_color = colors.next();
//let unexpected_color = colors.next();

//  for error in &err.errors {
//        match error {
//combine::easy::Error::Unexpected(info) => {
//      unexpected = format!(
//            "{}, {}",
//              unexpected,
//                unexpected_color.paint(info.to_string())
//              )
//            }
// combine::easy::Error::Expected(info) => {
//  expected = format!("{}, {}", expected, expected_color.paint(info.to_string()))
//   }
//     _ => (),
//   }
// }

//   expected = "Expected ".to_owned() + &expected;
//unexpected = "Unexpected ".to_owned() + &unexpected;

//  ariadne::Report::build(ariadne::ReportKind::Error, input_name, err.position)
//        .with_code("E0001")
//.with_message("Invalid syntax")
//  .with_label(
//        ariadne::Label::new((input_name, err.position..(err.position + 1)))
//              .with_message(unexpected)
//                .with_color(unexpected_color),
//  )
//    .with_note(expected)
//      .finish()
//        .eprint((input_name, Source:://from(input)))
//      .expect("something");
//
//  println!("{:?}", &err);
//}
