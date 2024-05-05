use std::{
    io::Read,
    path::{Path, PathBuf},
};

/// Finds all files that match the given glob pattern in the CWD.
pub fn find_files(matcher: globset::GlobMatcher) -> anyhow::Result<impl Iterator<Item = PathBuf>> {
    let cwd = std::env::current_dir()?;

    let files = jwalk::WalkDir::new(cwd)
        .try_into_iter()?
        .filter_map(|e| e.ok())
        .filter_map(move |e| {
            if e.path().is_file() && matcher.is_match(e.path()) {
                Some(e.path())
            } else {
                None
            }
        });

    Ok(files)
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
