use std::env;

pub fn init_logger() -> anyhow::Result<()> {
    env_logger::init();
    Ok(())
}
