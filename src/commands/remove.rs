use crate::config::{load_config, save_config};
use anyhow::Result;
use std::path::Path;

pub fn execute(config_path: &Path, name: String) -> Result<()> {
    let mut config = load_config(config_path)?;
    let before = config.remotes.len();
    config.remotes.retain(|remote| remote.name != name);
    if config.remotes.len() == before {
        println!("未找到匹配的远程仓库。");
        return Ok(());
    }
    save_config(config_path, &config)?;
    println!("已移除。");
    Ok(())
}
