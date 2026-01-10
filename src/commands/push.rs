use crate::git::{
    check_git_available, check_remote_available, current_branch, ensure_git_repo, git_remote_names,
    run_git_get_push_urls, run_git_push,
};
use anyhow::Result;
use std::path::Path;

const REMOTE_NAME: &str = "push-backup";

pub fn execute(_config_path: &Path, dry_run: bool) -> Result<()> {
    check_git_available()?;
    // config is not strictly needed for push as we rely on git remote configuration
    // let config = load_config(config_path)?;
    
    ensure_git_repo()?;
    let existing = git_remote_names()?;
    
    if !existing.contains(REMOTE_NAME) {
        println!("✗ 未找到统一远程仓库配置 '{}'，请先运行 apply <仓库名>", REMOTE_NAME);
        return Ok(());
    }

    let branch = current_branch()?;
    let urls = run_git_get_push_urls(REMOTE_NAME)?;

    if urls.is_empty() {
        println!("✗ 远程仓库 '{}' 未配置推送地址", REMOTE_NAME);
        return Ok(());
    }

    let mut success_count = 0;
    let mut fail_count = 0;

    for url in urls {
        // 在推送前检查远程仓库可用性
        if !dry_run {
            print!("检查远程仓库 '{}' 的可用性...", url);
            match check_remote_available(&url) {
                Ok(true) => println!(" ✓ 可访问"),
                Ok(false) => {
                    println!(" ✗ 无法访问，跳过推送");
                    fail_count += 1;
                    continue;
                }
                Err(e) => {
                    println!(" ✗ 检查失败: {}，跳过推送", e);
                    fail_count += 1;
                    continue;
                }
            }
        }

        if dry_run {
            println!("git push {} {}", url, branch);
        } else {
            match run_git_push(&url, &branch) {
                Ok(_) => {
                    println!(
                        "✓ 已将本地 {} 分支成功推送至 {}",
                        branch, url
                    );
                    success_count += 1;
                }
                Err(e) => {
                    println!("✗ 推送至 {} 失败: {}", url, e);
                    fail_count += 1;
                }
            }
        }
    }

    if !dry_run {
        println!("\n推送完成: {} 成功, {} 失败", success_count, fail_count);
    }
    Ok(())
}
