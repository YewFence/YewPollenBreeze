use crate::config::{load_config, save_config};
use anyhow::Result;
use std::path::Path;

pub fn execute(config_path: &Path, name: String) -> Result<()> {
    let mut config = load_config(config_path)?;
    let before = config.remotes.len();
    config.remotes.retain(|remote| remote.name != name);
    if config.remotes.len() == before {
        println!("No matching remote found.");
        return Ok(());
    }
    save_config(config_path, &config)?;
    println!("Removed.");
    Ok(())
}
