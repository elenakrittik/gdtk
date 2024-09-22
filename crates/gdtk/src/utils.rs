use std::{
    io::Read,
    path::{Path, PathBuf},
};

use itertools::Itertools;

use crate::cli::Cli;

pub fn setup_tracing(cli: &Cli) -> anyhow::Result<()> {
    let logs = gdtk_paths::logs_path()?;
    let writer = tracing_appender::rolling::daily(logs, "log");

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(cli.verbosity())
        .with_ansi(false)
        .with_writer(writer)
        .pretty()
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

pub fn resolve_files_by_ext(files: Vec<PathBuf>, ext: &str) -> anyhow::Result<Vec<PathBuf>> {
    if let [file] = files.as_slice() {
        if file.to_str().is_some_and(|p| p == "-") {
            return Ok(files);
        }
    }

    // this right here is how you SHOULD NOT design an API
    let walker = match files.as_slice() {
        [] => unreachable!(),
        [file] => {
            let mut builder = ignore::WalkBuilder::new(file.as_path());
            builder.hidden(false);
            builder.build()
        }
        files => {
            let mut files = files.iter();
            let mut builder = ignore::WalkBuilder::new(files.next().unwrap().as_path());

            builder.hidden(false);

            for file in files {
                builder.add(file.as_path());
            }

            builder.build()
        }
    };

    Ok(walker
        .filter_map(Result::ok)
        .map(|e| e.into_path())
        .filter(|p| p.extension().is_some_and(|e| e == ext))
        .unique()
        .collect())
}

pub fn get_content(file: &Path) -> anyhow::Result<String> {
    Ok(if file.to_str().is_some_and(|p| p == "-") {
        let mut buf = String::new();
        std::io::stdin().lock().read_to_string(&mut buf)?;

        buf
    } else {
        std::fs::read_to_string(file)?
    })
}
