use gdtk_gvm::utils::is_stable;
use itertools::Itertools;

pub async fn run(version: Option<String>) -> anyhow::Result<()> {
    let version = match version {
        Some(v) => v,
        None => prompt_version().await?,
    };

    eprintln!("selected: {version}");

    // let version = gdtk_gvm::versions::Versioning::new(&version)
    //     .ok_or(anyhow::anyhow!("Invalid Godot version: {version}"))?;
    // let versions = gdtk_gvm::online::fetch_versions().await?;

    // if !versions.contains(&version) {
    //     anyhow::bail!("{version} is an unknown Godot version.");
    // }

    // let mut local = gdtk_gvm::read_local_versions()?;
    // let target_dir = gdtk_gvm::godots_path()?.join(&version);

    // let old = local.insert(
    //     version.clone(),
    //     gdtk_gvm::toml::Value::String(target_dir.display().to_string()),
    // );

    // if old.is_some() {
    //     anyhow::bail!("{version} is already installed.");
    // }

    // println!("Downloading..");

    // let source = std::io::Cursor::new(gdtk_gvm::online::download_version_zip(&version).await?);

    // println!("Extracting..");

    // zip_extract::extract(source, &target_dir, true)?;

    // // Enable self-contained mode.
    // std::fs::File::create(target_dir.join("._sc_"))?;

    // gdtk_gvm::write_local_versions(local)?;

    // println!("Installed Godot {}!", version);

    Ok(())
}

async fn prompt_version() -> anyhow::Result<String> {
    let variant_dev = "Development versions..";
    let theme = dialoguer::theme::ColorfulTheme::default();

    let vers = vec!["4.2-stable", "4.1-stable", "3.6-dev0", "4.3-dev1"]
        .into_iter()
        .map(gdtk_gvm::versions::Versioning::new)
        .map(Option::unwrap)
        .collect::<Vec<_>>();

    let mut versions = (|| async { Ok::<_, anyhow::Error>(vers) })() // gdtk_gvm::online::fetch_versions()
        .await?
        .into_iter()
        .map(|ver| {
            if is_stable(&ver) {
                ("stable", ver.to_string())
            } else {
                ("dev", ver.to_string())
            }
        })
        .chain(std::iter::once(("stable", variant_dev.to_owned())))
        .into_group_map();

    let stables = versions.get_mut("stable").unwrap().as_mut_slice();

    let mut version = stables
        .get_mut(
            dialoguer::FuzzySelect::with_theme(&theme)
                .with_prompt("Select version to install")
                .max_length(7)
                .default(0)
                .items(stables)
                .interact()?,
        )
        .unwrap();

    if version == variant_dev {
        let devs = versions.get_mut("dev").unwrap().as_mut_slice();

        version = devs
            .get_mut(
                dialoguer::FuzzySelect::with_theme(&theme)
                    .with_prompt("Select version to install")
                    .max_length(7)
                    .default(0)
                    .items(devs)
                    .interact()?,
            )
            .unwrap();
    }

    Ok(std::mem::take(version))
}
