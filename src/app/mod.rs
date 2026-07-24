pub mod application;
mod logger;
mod resources;

pub fn initialize() -> anyhow::Result<()> {
    self::logger::init_logger()?;
    log::info!("Logger initialized");
    Ok(())
}
