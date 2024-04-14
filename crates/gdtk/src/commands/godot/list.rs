use tabled::settings::{format::Format, object::Cell, Style};

pub fn run() -> anyhow::Result<()> {
    let mut version_manager = gdtk_gvm::VersionManager::load()?;

    if version_manager.is_empty() {
        eprintln!("No versions installed.");
        return Ok(());
    }

    if let Some(default) = version_manager.versions.default {
        let def_ver = version_manager.versions.versions.remove(&default).unwrap();
        version_manager.versions.versions.insert(default + " (default)", def_ver);
    }

    let mut table = tabled::Table::new(version_manager.versions.versions);
    table.with(Style::modern_rounded());
    table.modify(Cell::new(0, 0), Format::content(|_| "Version".to_owned()));

    eprintln!("{table}");

    Ok(())
}

// async fn show_online_versions(old: bool, dev: bool) -> anyhow::Result<()> {
//     eprintln!("Available versions:");

//     let lim = gdtk_gvm::versions::Versioning::new("3.4").unwrap();

//     let versions_temp = gdtk_gvm::online::fetch_versions()
//         .await?
//         .into_iter()
//         .filter(|ver| dev || gdtk_gvm::utils::is_stable(ver))
//         .filter(|ver| old || ver >= &lim)
//         .group_by(major_minor_of_ver);

//     let versions = versions_temp.into_iter().map(|(k, vers)| {
//         (
//             k,
//             vers.map(|ver| ver.to_string().trim_end_matches("-stable").to_owned()),
//         )
//     });

//     if !(old || dev) {
//         for ((major, minor), vers) in versions.into_iter() {
//             eprintln!("  {}.{} ({})", major, minor, vers.last().unwrap())
//         }
//     } else {
//         for ((major, minor), mut vers) in versions.into_iter() {
//             eprintln!("  {}.{} ({})", major, minor, vers.join(", "))
//         }
//     }

//     if !(old && dev) {
//         eprintln!();
//     }

//     if !old {
//         eprintln!("hint: run with `--old` to show outdated versions");
//     }

//     if !dev {
//         eprintln!("hint: run with `--dev` to show development versions");
//     }

//     Ok(())
// }

// fn major_minor_of_ver(ver: &gdtk_gvm::versions::Versioning) -> (u32, u32) {
//     match ver {
//         gdtk_gvm::versions::Versioning::Ideal(ver) => (ver.major, ver.minor),
//         gdtk_gvm::versions::Versioning::General(gdtk_gvm::versions::Version {
//             chunks: gdtk_gvm::versions::Chunks(vec),
//             ..
//         }) => {
//             let chunk = vec.first_chunk::<2>().unwrap();
//             (
//                 chunk[0].single_digit().unwrap(),
//                 chunk[1].single_digit().unwrap(),
//             )
//         }
//         _ => panic!("unexpected version {ver:?}"),
//     }
// }
