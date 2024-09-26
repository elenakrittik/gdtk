use std::{
    fs::File,
    io::{Read, Seek},
};

use cliui::{Action, Prompt};
use gdtk_gvm::{
    online::{fetch_version_assets, fetch_versions},
    types::LocalVersion,
    utils::pick_asset,
    version::OnlineVersion,
};
use gdtk_paths::camino::Utf8Path;

pub struct GodotInstallCommand {
    version: OnlineVersion,
    mono: bool,
}

impl tapcli::Command for GodotInstallCommand {
    type Error = anyhow::Error;

    fn parse(_: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let (version, mono) = prompt_for_version()?;

        Ok(Self { version, mono })
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        let mut manager = gdtk_gvm::VersionManager::load()?;

        let display_version = format!(
            "{}{}",
            self.version.name(),
            if self.mono { "-mono" } else { "" }
        );
        let target_dir = gdtk_paths::godots_path()?.join(&display_version);

        if manager
            .get_version(self.version.name(), self.mono)
            .is_some()
        {
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

        manager.add_version(LocalVersion {
            name: self.version.name().to_owned(),
            path: target_dir.into_string(),
            mono: self.mono,
        });

        manager.save()?;

        spinner.success(&format!("Installed Godot {}!", &display_version));

        Ok(())
    }
}

fn extract_godot(source: impl Read + Seek, target_dir: &Utf8Path) -> anyhow::Result<()> {
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

fn prompt_for_version() -> anyhow::Result<(OnlineVersion, bool)> {
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
                    *prompt.state_mut() = !prompt.state();

                    let description = if *prompt.state() {
                        TOGGLE_MONO_DESC_YES
                    } else {
                        TOGGLE_MONO_DESC_NO
                    };

                    prompt
                        .actions_mut()
                        .get_mut(&TOGGLE_MONO_KEY)
                        .unwrap()
                        .description = description;

                    Ok(())
                },
            },
        )
        .allow_esc(false)
        .build()
        .interact()?;

    Ok((version.unwrap(), mono))
}
