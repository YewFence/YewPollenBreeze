use crate::config::load_config;
use crate::git::{
    check_git_available, check_remote_available, ensure_git_repo, git_remote_names,
    run_git_add_push_url, run_git_add_remote, run_git_get_remote_url, run_git_remote_remove,
};
use crate::utils::build_remote_url;
use anyhow::Result;
use std::env;
use std::io::{self, Write};
use std::path::Path;

const REMOTE_NAME: &str = "push-backup";

pub fn execute(config_path: &Path, repo: Option<String>, yes: bool, timeout: u64) -> Result<()> {
    check_git_available()?;
    let config = load_config(config_path)?;
    if config.remotes.is_empty() {
        println!("没有保存的远程仓库配置。");
        return Ok(());
    }
    ensure_git_repo()?;
    let existing = git_remote_names()?;

    // 确定仓库名称
    let repo = match repo {
        Some(name) => name,
        None => {
            let mut detected_name = None;

            // 1. 尝试从现有 remote 推断
            // 优先查找 origin，否则取任意一个非 push-backup 的 remote
            let remote_candidate = if existing.contains("origin") {
                Some("origin")
            } else {
                existing
                    .iter()
                    .find(|&n| n != REMOTE_NAME)
                    .map(|s| s.as_str())
            };

            if let Some(remote) = remote_candidate {
                if let Ok(url) = run_git_get_remote_url(remote) {
                    let url = url.trim();
                    let url = url.strip_suffix(".git").unwrap_or(url);
                    if let Some(name) = url.rsplit('/').next() {
                        if !name.is_empty() {
                            detected_name = Some(name.to_string());
                        }
                    }
                }
            }

            // 2. 尝试从目录名推断
            if detected_name.is_none() {
                if let Ok(cwd) = env::current_dir() {
                    if let Some(name) = cwd.file_name() {
                        detected_name = Some(name.to_string_lossy().to_string());
                    }
                }
            }

            let name =
                detected_name.ok_or_else(|| anyhow::anyhow!("无法自动检测仓库名称，请手动指定"))?;

            println!("检测到仓库名称为: {}", name);
            if !yes {
                println!("提示: 你也可以通过 'push-backup apply <name>' 手动指定名称");
                print!("确认使用此名称吗? (y/n) ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let input = input.trim();

                if !input.eq_ignore_ascii_case("y") && !input.eq_ignore_ascii_case("yes") {
                    println!("操作已取消。");
                    return Ok(());
                }
            }
            name
        }
    };

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
        match check_remote_available(&url, timeout) {
            Ok(true) => println!(" ✓ 可访问"),
            Ok(false) => println!(" ✗ 无法访问（可能需要配置认证或网络不通）"),
            Err(e) => println!(" ✗ 检查失败: {}", e),
        }
    }

    Ok(())
}
