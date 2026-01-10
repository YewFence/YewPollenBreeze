use crate::config::load_config;
use anyhow::Result;
use std::path::Path;

/// 截断字符串，超过指定长度时添加省略号
fn truncate_note(note: &str, max_len: usize) -> String {
    if note.chars().count() <= max_len {
        note.to_string()
    } else {
        format!("{}...", note.chars().take(max_len).collect::<String>())
    }
}

pub fn execute(config_path: &Path, long: bool) -> Result<()> {
    let config = load_config(config_path)?;
    if config.remotes.is_empty() {
        println!("没有保存的远程仓库配置。");
        return Ok(());
    }

    if long {
        // 详细模式：分段显示完整信息
        println!("=== 所有远程仓库配置 ===");
        println!("共 {} 个配置\n", config.remotes.len());
        for (i, remote) in config.remotes.iter().enumerate() {
            println!("[{}] {}", i + 1, remote.name);
            println!("    基础地址: {}", remote.base);
            if let Some(note) = &remote.note {
                println!("    备注: {}", note);
            }
            if i < config.remotes.len() - 1 {
                println!();
            }
        }
    } else {
        // 简洁模式：截断 note
        for remote in config.remotes {
            if let Some(note) = remote.note {
                let truncated = truncate_note(&note, 20);
                println!("{}\t{}\t# {}", remote.name, remote.base, truncated);
            } else {
                println!("{}\t{}", remote.name, remote.base);
            }
        }
    }
    Ok(())
}
