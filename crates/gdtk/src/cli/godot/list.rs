use tabled::settings::{format::Format, object::Cell, Style};

pub struct GodotListCommand;

impl tapcli::Command for GodotListCommand {
    type Error = anyhow::Error;

    async fn parse(_: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        Ok(Self)
    }

    async fn run(self) -> Result<Self::Output, Self::Error> {
        let mut manager = gdtk_gvm::VersionManager::load()?;

        if manager.is_empty() {
            eprintln!("No versions installed.");
            return Ok(());
        }

        if let Some(default) = manager.inner.default {
            let def_ver = manager.inner.versions.remove(&default).unwrap();
            manager
                .inner
                .versions
                .insert(default + " (default)", def_ver);
        }

        let mut table = tabled::Table::new(manager.inner.versions);
        table.with(Style::modern_rounded());
        table.modify(Cell::new(0, 0), Format::content(|_| "Version".to_owned()));

        eprintln!("{table}");

        Ok(())
    }
}
