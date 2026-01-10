use crate::config::{load_config, save_config, Config};
use anyhow::{bail, Result};
use std::path::{Path, PathBuf};

/// 候选配置文件名
const CANDIDATE_FILES: &[&str] = &["config.toml", "push-backup.toml"];

/// 查找当前目录下的候选配置文件
fn find_candidate_files() -> Vec<PathBuf> {
    CANDIDATE_FILES
        .iter()
        .map(PathBuf::from)
        .filter(|p| p.exists())
        .collect()
}

pub fn execute(config_path: &Path, input: Option<PathBuf>, merge: bool) -> Result<()> {
    let input = match input {
        Some(path) => path,
        None => {
            let candidates = find_candidate_files();
            if candidates.is_empty() {
                println!("未指定导入文件。");
                println!();
                println!("用法: pb config import -i <文件路径>");
                println!();
                println!("提示: 当前目录下未找到 {} 文件。", CANDIDATE_FILES.join(" 或 "));
                return Ok(());
            } else if candidates.len() == 1 {
                let file = &candidates[0];
                println!(
                    "提示: 检测到当前目录存在 {}，你是想执行以下命令吗？",
                    file.display()
                );
                println!();
                println!("  pb config import -i {}", file.display());
                if merge {
                    println!("  pb config import -i {} --merge", file.display());
                }
                return Ok(());
            } else {
                println!("未指定导入文件，但检测到当前目录存在以下候选文件：");
                println!();
                for file in &candidates {
                    println!("  pb config import -i {}", file.display());
                }
                println!();
                println!("请选择一个文件进行导入。");
                return Ok(());
            }
        }
    };

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
