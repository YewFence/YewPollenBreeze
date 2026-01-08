use crate::config::load_config;
use anyhow::Result;
use std::path::Path;

pub fn execute(config_path: &Path, name: Option<String>) -> Result<()> {
    let config = load_config(config_path)?;
    if config.remotes.is_empty() {
        println!("没有保存的远程仓库配置。");
        return Ok(());
    }

    match name {
        Some(name) => {
            // 显示单个配置的详细信息
            let remote = config.remotes.iter().find(|r| r.name == name);
            match remote {
                Some(r) => {
                    println!("=== 远程仓库详情 ===");
                    println!("名称: {}", r.name);
                    println!("基础URL: {}", r.base);
                }
                None => {
                    println!("未找到名为 '{}' 的远程仓库配置。", name);
                }
            }
        }
        None => {
            // 显示所有配置的详细信息
            println!("=== 所有远程仓库配置 ===");
            println!("共 {} 个配置\n", config.remotes.len());
            for (i, remote) in config.remotes.iter().enumerate() {
                println!("[{}] {}", i + 1, remote.name);
                println!("    基础URL: {}", remote.base);
                if i < config.remotes.len() - 1 {
                    println!();
                }
            }
        }
    }
    Ok(())
}
