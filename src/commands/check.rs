use crate::config::{load_config, Config};
use crate::git::{
    check_git_available, check_remote_available, ensure_git_repo, git_remote_names,
    run_git_get_push_urls,
};
use anyhow::Result;
use std::path::Path;

const REMOTE_NAME: &str = "push-backup";

pub fn execute(config_path: &Path, timeout: u64) -> Result<()> {
    check_git_available()?;
    ensure_git_repo()?;

    // 检查 push-backup 远程是否存在
    let remotes = git_remote_names()?;
    if !remotes.contains(REMOTE_NAME) {
        println!("错误: 未找到 push-backup 远程仓库。");
        println!("提示: 请先运行 `push-backup apply` 应用配置。");
        return Ok(());
    }

    let urls = run_git_get_push_urls(REMOTE_NAME)?;

    if urls.is_empty() {
        println!("错误: 远程仓库 '{}' 未配置推送地址。", REMOTE_NAME);
        return Ok(());
    }

    let config = load_config(config_path)?;

    let mut success_count = 0;
    let mut fail_count = 0;

    for url in urls {
        let name = match_config_name(&config, &url);

        print!("{:12} ", format!("{}:", name));

        match check_remote_available(&url, timeout) {
            Ok(true) => {
                println!("✓ 连接正常");
                success_count += 1;
            }
            Ok(false) => {
                println!("✗ 连接失败");
                fail_count += 1;
            }
            Err(e) => {
                println!("✗ 检查失败: {}", e);
                fail_count += 1;
            }
        }
    }

    println!("\n检查完成: {} 成功, {} 失败", success_count, fail_count);

    Ok(())
}

/// 匹配 URL 对应的配置名称
fn match_config_name(config: &Config, url: &str) -> String {
    // 按 base 长度降序排序，确保最长前缀匹配
    let mut remotes = config.remotes.clone();
    remotes.sort_by(|a, b| b.base.len().cmp(&a.base.len()));

    for remote in &remotes {
        if url.starts_with(&remote.base) {
            let remainder = &url[remote.base.len()..];
            if remainder.is_empty()
                || remote.base.ends_with('/')
                || remote.base.ends_with(':')
                || remainder.starts_with('/')
                || remainder.starts_with(':')
            {
                return remote.name.clone();
            }
        }
    }

    "未命名".to_string()
}
