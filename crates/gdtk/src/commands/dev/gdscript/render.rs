use diagnosis::{visualizers::rustc::RustcVisualizer, Diagnostic, Highlight, Severity, Visualizer};

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
            Highlight::new(&(21..44)).with_message("this branch is useless, just like myself"),
        )
        .add_highlight(Highlight::new(&(1..8)).with_message("condition always true"))
        .add_help("this isn't helpful, is it?");

    let renderer = RustcVisualizer::new("<const>", SOURCE);

    renderer.visualize(diagnostic, &mut stderr)?;

    Ok(())
}
