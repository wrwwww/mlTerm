mod actions;
pub mod application;
mod bootstrap;
mod commands;
pub mod config;
pub mod event_bus;
mod logger;
mod resources;

pub fn initialize() -> anyhow::Result<()> {
    self::logger::init_logger()?;
    log::info!("Logger initialized");
    Ok(())
}
