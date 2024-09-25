pub struct GodotListCommand;

impl tapcli::Command for GodotListCommand {
    type Error = anyhow::Error;

    async fn parse(_: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        Ok(Self)
    }

    async fn run(self) -> Result<Self::Output, Self::Error> {
        let manager = gdtk_gvm::VersionManager::load()?;

        if manager.is_empty() {
            eprintln!("No versions installed.");
            return Ok(());
        }

        let table = tabled::Table::new(manager.installed());

        eprintln!("{table}");

        Ok(())
    }
}
