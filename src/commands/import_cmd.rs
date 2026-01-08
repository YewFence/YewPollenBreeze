use crate::config::{load_config, save_config, Config};
use anyhow::{bail, Result};
use std::path::{Path, PathBuf};

pub fn execute(config_path: &Path, input: PathBuf, merge: bool) -> Result<()> {
    if !input.exists() {
        bail!("导入文件不存在: {}", input.display());
    }

    let import_config = load_config(&input)?;
    if import_config.remotes.is_empty() {
        println!("导入文件中没有配置。");
        return Ok(());
    }

    let mut config = if merge {
        load_config(config_path)?
    } else {
        Config::default()
    };

    let mut added = 0;
    let mut updated = 0;

    for import_remote in import_config.remotes {
        let mut found = false;
        for remote in &mut config.remotes {
            if remote.name == import_remote.name {
                remote.base = import_remote.base.clone();
                updated += 1;
                found = true;
                break;
            }
        }
        if !found {
            config.remotes.push(import_remote);
            added += 1;
        }
    }

    save_config(config_path, &config)?;

    if merge {
        println!("配置已合并: 新增 {} 个，更新 {} 个。", added, updated);
    } else {
        println!("配置已导入: {} 个远程仓库。", added);
    }
    Ok(())
}
