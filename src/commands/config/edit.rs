use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn execute(config_path: &Path) -> Result<()> {
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("创建配置目录失败: {}", parent.display()))?;
    }

    if !config_path.exists() {
        // 创建空文件，避免编辑器打开失败
        fs::write(config_path, "")
            .with_context(|| format!("创建配置文件失败: {}", config_path.display()))?;
    }

    edit::edit_file(config_path)?;
    Ok(())
}
