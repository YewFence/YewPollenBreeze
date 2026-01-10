use crate::config::load_config;
use crate::git::{
    check_git_available, check_remote_available, ensure_git_repo, git_remote_names,
    run_git_add_push_url, run_git_add_remote, run_git_remote_remove,
};
use crate::utils::build_remote_url;
use anyhow::Result;
use std::path::Path;

const REMOTE_NAME: &str = "push-backup";

pub fn execute(config_path: &Path, repo: String) -> Result<()> {
    check_git_available()?;
    let config = load_config(config_path)?;
    if config.remotes.is_empty() {
        println!("没有保存的远程仓库配置。");
        return Ok(());
    }
    ensure_git_repo()?;
    let existing = git_remote_names()?;

    // 1. 清理旧的独立远程仓库（如果存在）
    for remote in &config.remotes {
        if existing.contains(&remote.name) && remote.name != REMOTE_NAME {
            run_git_remote_remove(&remote.name)?;
            println!("已清理旧远程仓库: {}", remote.name);
        }
    }

    // 2. 重置 push-backup 远程仓库
    if existing.contains(REMOTE_NAME) {
        run_git_remote_remove(REMOTE_NAME)?;
    }

    // 计算所有 URL
    let mut remote_urls = Vec::new();
    for remote in &config.remotes {
        let url = build_remote_url(&remote.base, &repo);
        remote_urls.push((remote.name.clone(), url));
    }

    // 3. 创建 push-backup 远程仓库
    // 使用第一个 URL 作为 fetch URL
    if let Some((_, first_url)) = remote_urls.first() {
        run_git_add_remote(REMOTE_NAME, first_url)?;
        println!("已配置统一远程仓库: {}", REMOTE_NAME);
    }

    // 4. 添加所有 push URL 并检查可用性
    for (name, url) in remote_urls {
        // 添加 push URL
        run_git_add_push_url(REMOTE_NAME, &url)?;

        // 检查可用性 (使用 URL 进行检查)
        print!("检查远程仓库 '{}' ({}) 的可用性...", name, url);
        match check_remote_available(&url) {
            Ok(true) => println!(" ✓ 可访问"),
            Ok(false) => println!(" ✗ 无法访问（可能需要配置认证或网络不通）"),
            Err(e) => println!(" ✗ 检查失败: {}", e),
        }
    }

    Ok(())
}
