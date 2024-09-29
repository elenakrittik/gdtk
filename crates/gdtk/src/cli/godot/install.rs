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
use gdtk_paths::camino::{Utf8Path, Utf8PathBuf};

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

        extract_godot(&mut source, &target_dir, self.mono)?;

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

fn extract_godot(
    source: impl Read + Seek,
    target_dir: &Utf8Path,
    unwrap_top_dir: bool,
) -> anyhow::Result<()> {
    let _ = gdtk_paths::ensure_dir_exists(target_dir)?;

    let mut archive = zip::ZipArchive::new(source)?;

    for n in 0..archive.len() {
        let mut entry = archive.by_index(n)?;

        let entry_path = entry.enclosed_name().map(Utf8PathBuf::try_from).unwrap()?;
        let entry_path = normalize_entry_path(entry_path, unwrap_top_dir);

        // "hey PathBuf can you join "./somepath/" and "hello\world"" - "sure thing, tho slashes are your problem, sucks to suck"
        // explanation: PathBuf apparently does not even normalize slashes when joining. therefore we do the stupid thing.
        let mut full = target_dir.to_owned();

        full.extend(entry_path.components());

        if entry.is_dir() {
            std::fs::create_dir_all(&full)?;
            continue;
        }

        let mut entry_target = File::create(&full)?;

        std::io::copy(&mut entry, &mut entry_target)?;
    }

    Ok(())
}

fn normalize_entry_path(path: Utf8PathBuf, unwrap_top_dir: bool) -> Utf8PathBuf {
    let path = if unwrap_top_dir {
        let mut c = path.components();
        c.next();
        c.as_path().to_owned()
    } else {
        path
    };

    // if it is a top-level entry
    if path.components().count() == 1 {
        let is_godot = path.as_str().contains("Godot_v");
        let is_console = path.as_str().contains("console");

        if is_godot && is_console {
            Utf8PathBuf::from("godot_console")
        } else if is_godot && !is_console {
            Utf8PathBuf::from("godot")
        } else {
            path
        }
    } else {
        path
    }
}

const TOGGLE_MONO_KEY: cliui::Key = cliui::Key::Char('m');
const TOGGLE_MONO_DESC: &str = "Install the mono variant?";

fn prompt_for_version() -> anyhow::Result<(OnlineVersion, bool)> {
    let available_versions = fetch_versions()?;

    let (version, mono) = Prompt::builder()
        .with_question("Select version")
        .with_state(false)
        .with_items(available_versions)
        .with_action(
            TOGGLE_MONO_KEY,
            Action {
                description: TOGGLE_MONO_DESC,
                callback: |prompt| {
                    *prompt.state_mut() = !prompt.state();

                    Ok(())
                },
            },
        )
        .allow_esc(false)
        .build()
        .interact()?;

    Ok((version.unwrap(), mono))
}
