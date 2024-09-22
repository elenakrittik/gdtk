use std::{
    fs::File,
    io::{Read, Seek},
    path::Path,
};

use cliui::{Action, Prompt};
use gdtk_gvm::{
    online::{fetch_version_assets, fetch_versions},
    utils::pick_asset,
    version::Version,
};

use crate::cli::godot::symlink_default_version;

pub struct GodotInstallCommand {
    version: Version,
    mono: bool,
}

impl tapcli::Command for GodotInstallCommand {
    type Error = anyhow::Error;

    async fn parse(_: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let (version, mono) = prompt_for_version()?;

        Ok(Self { version, mono })
    }

    async fn run(self) -> Result<Self::Output, Self::Error> {
        let mut manager = gdtk_gvm::VersionManager::load()?;

        let set_as_default = manager.is_empty();
        let target_dir = gdtk_paths::godots_path()?.join(self.version.name());

        let already_installed = manager.add_version(
            self.version.name(),
            gdtk_gvm::types::Version {
                path: target_dir.clone(),
            },
        );

        if already_installed {
            anyhow::bail!("Godot {} is already installed.", self.version.name());
        }

        let assets = fetch_version_assets(self.version.name())?;
        let asset = pick_asset(&assets, self.mono)
            .expect("Couldn't find a Godot build for current OS/arch pair.");

        let mut spinner = spinoff::Spinner::new(
            spinoff::spinners::Dots2,
            "Downloading (this may take a while)..",
            spinoff::Color::Cyan,
        );

        let mut content = vec![];

        ureq::get(&asset.download_url.0)
            .call()?
            .into_body()
            .into_reader()
            .read_to_end(&mut content)?;

        let mut source = std::io::Cursor::new(content);

        spinner.update_text("Extracting..");

        extract_godot(&mut source, &target_dir)?;

        spinner.update_text("Setting up..");

        // Enable self-contained mode.
        std::fs::File::create(target_dir.join("._sc_"))?;

        if set_as_default {
            manager.inner.default = Some(self.version.name().to_string());
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

const TOGGLE_MONO_KEY: cliui::Key = cliui::Key::Char('m');
const TOGGLE_MONO_DESC_NO: &str = "Install the mono variant? (current: no)";
const TOGGLE_MONO_DESC_YES: &str = "Install the mono variant? (current: yes)";

fn prompt_for_version() -> anyhow::Result<(Version, bool)> {
    let available_versions = fetch_versions()?;

    let (version, mono) = Prompt::builder()
        .with_question("Select version")
        .with_items(available_versions)
        .with_state(false)
        .with_action(
            TOGGLE_MONO_KEY,
            Action {
                description: TOGGLE_MONO_DESC_NO,
                callback: |prompt| {
                    prompt.state = !prompt.state;

                    let action = prompt.actions.get_mut(&TOGGLE_MONO_KEY).unwrap();

                    if prompt.state {
                        action.description = TOGGLE_MONO_DESC_YES;
                    } else {
                        action.description = TOGGLE_MONO_DESC_NO;
                    }

                    Ok(())
                },
            },
        )
        .allow_esc(false)
        .build()
        .interact()?;

    Ok((version.unwrap(), mono))
}
