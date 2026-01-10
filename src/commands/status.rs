use crate::config::{load_config, Config};
use crate::git::{
    check_git_available, current_branch, ensure_git_repo, git_count_ahead_behind,
    git_ls_remote_ref, git_remote_names, run_git_get_push_urls,
};
use anyhow::Result;
use std::path::Path;

const REMOTE_NAME: &str = "yewpb";

pub fn execute(config_path: &Path) -> Result<()> {
    check_git_available()?;
    ensure_git_repo()?;

    // 检查 yewpb 远程是否存在
    let remotes = git_remote_names()?;
    if !remotes.contains(REMOTE_NAME) {
        println!("错误: 未找到 yewpb 远程仓库。");
        println!("提示: 请先运行 `yewpb apply` 应用配置。");
        return Ok(());
    }

    let branch = current_branch()?;
    let urls = run_git_get_push_urls(REMOTE_NAME)?;

    if urls.is_empty() {
        println!("错误: 远程仓库 '{}' 未配置推送地址。", REMOTE_NAME);
        return Ok(());
    }

    let config = load_config(config_path)?;

    println!("分支: {}\n", branch);

    for url in urls {
        let name = match_config_name(&config, &url);
        print_sync_status(&name, &url, &branch)?;
    }

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

/// 打印同步状态
fn print_sync_status(name: &str, url: &str, branch: &str) -> Result<()> {
    // 获取远程分支的 commit hash
    match git_ls_remote_ref(url, branch)? {
        Some(remote_commit) => {
            let (ahead, behind) = git_count_ahead_behind(&remote_commit)?;
            if ahead == 0 && behind == 0 {
                println!("{:12} ✓ 已同步", format!("{}:", name));
            } else {
                let mut status_parts = Vec::new();
                if ahead > 0 {
                    status_parts.push(format!("领先 {} 个提交", ahead));
                }
                if behind > 0 {
                    status_parts.push(format!("落后 {} 个提交", behind));
                }
                println!(
                    "{:12} ↑{} ↓{} ({})",
                    format!("{}:", name),
                    ahead,
                    behind,
                    status_parts.join(", ")
                );
            }
        }
        None => {
            println!(
                "{:12} ✗ 无法获取远程状态 (分支不存在或连接失败)",
                format!("{}:", name)
            );
        }
    }

    Ok(())
}
