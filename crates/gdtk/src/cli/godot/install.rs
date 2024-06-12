use std::{
    fs::File,
    io::{Read, Seek},
    path::Path,
};

use gdtk_gvm::{online::version_download_urls, utils::arch_os, versions::Versioning};

use crate::cli::{godot::symlink_default_version, utils::ParserExt};

pub struct GodotInstallCommand {
    pub version: Versioning,
}

impl tapcli::Command for GodotInstallCommand {
    type Error = anyhow::Error;

    async fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let pool = gdtk_gvm::online::fetch_versions().await?;
        let version = parser.next_godot_version(pool)?;

        Ok(Self { version })
    }

    async fn run(self) -> Result<Self::Output, Self::Error> {
        let mut manager = gdtk_gvm::VersionManager::load()?;
        let version_string = self.version.to_string();
        let set_as_default = manager.is_empty();
        let target_dir = gdtk_paths::godots_path()?.join(&version_string);

        let already_installed = manager.add_version(
            &version_string,
            gdtk_gvm::types::Version {
                path: target_dir.clone(),
            },
        );

        if already_installed {
            anyhow::bail!("Godot {version_string} is already installed.");
        }

        let download_urls = version_download_urls(&self.version, &version_string).await?;
        let url = download_urls.get(&arch_os()).ok_or(anyhow::anyhow!(
            "Couldn't find download URL for current arch/os pair {:?}.",
            arch_os(),
        ))?;

        let mut spinner = spinoff::Spinner::new(
            spinoff::spinners::Dots2,
            "Downloading (this may take a while)..",
            spinoff::Color::Cyan,
        );

        let content = reqwest::get(url.to_owned()).await?.bytes().await?;
        let mut source = std::io::Cursor::new(content);

        spinner.update_text("Extracting..");

        extract_godot(&mut source, &target_dir)?;

        spinner.update_text("Setting up..");

        // Enable self-contained mode.
        std::fs::File::create(target_dir.join("._sc_"))?;

        if set_as_default {
            manager.inner.default = Some(version_string);
            symlink_default_version(&target_dir)?;
        }

        manager.save()?;

        spinner.success(&format!("Installed Godot {}!", self.version));

        Ok(())
    }
}

fn extract_godot(source: impl Read + Seek, target_dir: &Path) -> zip::result::ZipResult<()> {
    gdtk_paths::ensure_path(target_dir, true)?;

    let mut archive = zip::ZipArchive::new(source)?;

    for n in 0..archive.len() {
        let mut file = archive.by_index(n)?;

        let mut path = File::create(if file.name().contains("console") {
            target_dir.join("godot_console")
        } else {
            target_dir.join("godot")
        })?;

        std::io::copy(&mut file, &mut path)?;
    }

    Ok(())
}
