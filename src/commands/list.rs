use crate::config::load_config;
use anyhow::Result;
use std::path::Path;

pub fn execute(config_path: &Path) -> Result<()> {
    let config = load_config(config_path)?;
    if config.remotes.is_empty() {
        println!("没有保存的远程仓库配置。");
        return Ok(());
    }
    for remote in config.remotes {
        println!("{}\t{}", remote.name, remote.base);
    }
    Ok(())
}
