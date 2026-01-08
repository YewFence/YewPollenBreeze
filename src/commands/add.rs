use crate::config::{load_config, save_config, Remote};
use anyhow::Result;
use std::path::Path;

pub fn execute(config_path: &Path, name: String, base: String) -> Result<()> {
    let mut config = load_config(config_path)?;
    let mut updated = false;
    for remote in &mut config.remotes {
        if remote.name == name {
            remote.base = base.clone();
            updated = true;
            break;
        }
    }
    if !updated {
        config.remotes.push(Remote { name, base });
    }
    save_config(config_path, &config)?;
    println!("已保存。");
    Ok(())
}
