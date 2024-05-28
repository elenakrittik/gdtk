#[derive(Default)]
pub(crate) struct CliContext {
    pub(crate) verbosity: u8,
}

impl CliContext {
    pub(crate) fn print_version_and_exit(&self) {
        println!("GDtk {}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }
}
