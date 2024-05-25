use diagnosis::{visualizers::rustc::RustcVisualizer, Diagnostic, Highlight, Severity, Visualizer};

// make sure to use LF instead of CRLF before saving the file
const SOURCE: &str = "
if true:
    return
else:
    print('hello')
";

pub fn run() -> anyhow::Result<()> {
    let mut stderr = std::io::stderr().lock();

    let diagnostic = Diagnostic::new("oops", Severity::Custom("custom"))
        .with_code("random-diag")
        .with_span(&(1..3))
        .add_highlight(
            Highlight::new(&(21..45)).with_message("this branch is useless, just like myself"),
        )
        .add_highlight(Highlight::new(&(4..8)).with_message("condition always true"))
        .add_highlight(Highlight::new(&(14..16)).with_message("re"))
        .add_highlight(Highlight::new(&(16..19)).with_message("turn"))
        .add_highlight(Highlight::new(&(31..36)).with_message("print"))
        .add_highlight(Highlight::new(&(36..37)).with_message("unmatched paren"))
        .add_help("this isn't helpful, is it?");

    let renderer = RustcVisualizer::new("<const>", SOURCE);

    renderer.visualize(diagnostic, &mut stderr)?;

    Ok(())
}
