use crate::config::load_config;
use crate::git::{
    check_git_available, check_remote_available, current_branch, ensure_git_repo, git_remote_names,
    run_git_push,
};
use anyhow::Result;
use std::path::Path;

pub fn execute(config_path: &Path, dry_run: bool) -> Result<()> {
    check_git_available()?;
    let config = load_config(config_path)?;
    if config.remotes.is_empty() {
        println!("No remotes saved.");
        return Ok(());
    }
    ensure_git_repo()?;
    let existing = git_remote_names()?;
    let branch = current_branch()?;

    let mut success_count = 0;
    let mut fail_count = 0;

    for remote in config.remotes {
        if !existing.contains(&remote.name) {
            println!(
                "✗ 远程仓库 '{}' 未在本地配置，请先运行 apply <repo>",
                remote.name
            );
            fail_count += 1;
            continue;
        }

        // 在推送前检查远程仓库可用性
        if !dry_run {
            print!("检查远程仓库 '{}' 的可用性...", remote.name);
            match check_remote_available(&remote.name) {
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
            println!("git push {} {}", remote.name, branch);
        } else {
            match run_git_push(&remote.name, &branch) {
                Ok(_) => {
                    println!(
                        "✓ 已将本地 {} 分支成功推送至 {} 的 {} 分支",
                        branch, remote.name, branch
                    );
                    success_count += 1;
                }
                Err(e) => {
                    println!("✗ 推送至 {} 失败: {}", remote.name, e);
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
