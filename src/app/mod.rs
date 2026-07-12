mod actions;
pub mod application;
mod bootstrap;
mod commands;
pub mod config;
mod logger;
mod resources;
mod settings;

pub fn initialize() -> anyhow::Result<()> {
    self::logger::init_logger()?;
    log::info!("Logger initialized");
    Ok(())
}
