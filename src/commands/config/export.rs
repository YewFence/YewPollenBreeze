use crate::config::{load_config, save_config};
use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn execute(config_path: &Path, output: Option<PathBuf>) -> Result<()> {
    let config = load_config(config_path)?;
    if config.remotes.is_empty() {
        println!("没有可导出的配置。");
        return Ok(());
    }

    let export_path = match output {
        Some(path) => path,
        None => PathBuf::from("yewpb-config.toml"),
    };

    save_config(&export_path, &config)?;
    println!("配置已导出到: {}", export_path.display());
    Ok(())
}
