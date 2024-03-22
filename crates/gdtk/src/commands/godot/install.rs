use gdtk_gvm::utils::is_stable;
use itertools::Itertools;

pub async fn run(version: Option<String>) -> anyhow::Result<()> {
    let version = match version {
        Some(v) => {
            let versioning = gdtk_gvm::versions::Versioning::new(&v)
                .ok_or(anyhow::anyhow!("Invalid Godot version: {v}"))?;
            let versions = gdtk_gvm::online::fetch_versions().await?;

            if !versions.contains(&versioning) {
                anyhow::bail!("{versioning} is an unknown Godot version.");
            }

            v
        }
        None => prompt_version().await?,
    };

    let mut version_manager = gdtk_gvm::VersionManager::load()?;
    let target_dir = gdtk_gvm::utils::godots_path()?.join(&version);

    let already_installed = version_manager.add_version(
        version.clone(),
        gdtk_gvm::types::Version {
            path: target_dir.clone(),
        },
    );

    if already_installed {
        anyhow::bail!("{version} is already installed.");
    }

    let arch = (std::env::consts::ARCH, std::env::consts::OS);

    let version_download_urls = gdtk_gvm::online::version_download_urls(&version).await?;
    let url = match version_download_urls.get(&arch) {
        Some(url) => url,
        None => anyhow::bail!("Couldn't find download URL for current arch/os pair."),
    };

    eprintln!("Downloading (this may take a while)..");

    let content = reqwest::get(url.to_owned()).await?.bytes().await?;
    let source = std::io::Cursor::new(content);

    eprintln!("Extracting..");

    zip_extract::extract(source, &target_dir, true)?;

    // Enable self-contained mode.
    std::fs::File::create(target_dir.join("._sc_"))?;

    version_manager.save()?;

    eprintln!("Installed Godot {}!", version);

    Ok(())
}

async fn prompt_version() -> anyhow::Result<String> {
    let variant_dev = "Development versions..";
    let theme = gdtk_dialoguer::theme::ColorfulTheme::default();

    let vers = gdtk_gvm::online::fetch_versions().await?;

    let mut versions = vers
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
            gdtk_dialoguer::FuzzySelect::new()
                .with_theme(&theme)
                .with_prompt("Select version to install")
                .with_max_length(7)
                .with_default(0)
                .add_items(stables)
                .interact()?,
        )
        .unwrap();

    if version == variant_dev {
        let devs = versions.get_mut("dev").unwrap().as_mut_slice();

        version = devs
            .get_mut(
                gdtk_dialoguer::FuzzySelect::new()
                    .with_theme(&theme)
                    .with_prompt("Select version to install")
                    .with_max_length(7)
                    .with_default(0)
                    .add_items(devs)
                    .interact()?,
            )
            .unwrap();
    }

    Ok(std::mem::take(version))
}
