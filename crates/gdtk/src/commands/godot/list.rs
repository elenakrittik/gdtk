use itertools::Itertools;
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, ContentArrangement, Row, Table,
};

pub async fn run(
    online: bool,
    old: bool,
    dev: bool,
) -> anyhow::Result<()> {
    if online {
        show_online_versions(old, dev).await?;
    } else {
        show_local_versions()?;
    }

    Ok(())
}

fn show_local_versions() -> anyhow::Result<()> {
    gdtk_gvm::ensure_versions()?;

    let versions = gdtk_gvm::read_local_versions()?;

    if versions.is_empty() {
        println!("No versions installed.");
        return Ok(());
    }

    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(["Version", "Path"])
        .add_rows(
            versions
                .into_iter()
                .map(|(v, p)| Row::from([v, p.as_str().unwrap().to_owned()])),
        );

    println!("{table}");

    Ok(())
}

async fn show_online_versions(
    old: bool,
    dev: bool,
) -> anyhow::Result<()> {
    println!("Available versions:");

    let lim = gdtk_gvm::versions::Versioning::new("3.4").unwrap();

    let versions_temp = gdtk_gvm::online::fetch_versions()
        .await?
        .into_iter()
        .filter(|ver| dev || gdtk_gvm::is_stable(ver))
        .filter(|ver| old || ver >= &lim )
        .group_by(major_minor_of_ver);

    let versions = versions_temp
        .into_iter()
        .map(|(k, vers)| (k, vers.map(|ver| ver.to_string().trim_end_matches("-stable").to_owned())));

    if !(old || dev) {
        for ((major, minor), vers) in versions.into_iter() {
            eprintln!("  {}.{} ({})", major, minor, vers.last().unwrap())
        }
    } else {
        for ((major, minor), mut vers) in versions.into_iter() {
            eprintln!("  {}.{} ({})", major, minor, vers.join(", "))
        }
    }

    Ok(())
}

fn major_minor_of_ver(ver: &gdtk_gvm::versions::Versioning) -> (u32, u32) {
    match ver {
        gdtk_gvm::versions::Versioning::Ideal(ver) => (ver.major, ver.minor),
        gdtk_gvm::versions::Versioning::General(
            gdtk_gvm::versions::Version {
                chunks: gdtk_gvm::versions::Chunks(vec),
                ..
            }
        ) => {
            let chunk = vec.first_chunk::<2>().unwrap();
            (chunk[0].single_digit().unwrap(), chunk[1].single_digit().unwrap())
        },
        _ => panic!("unexpected version {ver:?}"),
    }
}
