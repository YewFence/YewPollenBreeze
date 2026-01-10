use crate::config::load_config;
use crate::git::{
    check_git_available, check_remote_available, current_branch, ensure_git_repo, git_remote_names,
    run_git_get_push_urls, run_git_push, PushOptions,
};
use anyhow::Result;
use std::path::Path;

const REMOTE_NAME: &str = "push-backup";

pub fn execute(
    config_path: &Path,
    dry_run: bool,
    only: Vec<String>,
    except: Vec<String>,
    options: &PushOptions,
) -> Result<()> {
    check_git_available()?;
    let config = load_config(config_path)?;

    ensure_git_repo()?;
    let existing = git_remote_names()?;

    if !existing.contains(REMOTE_NAME) {
        println!(
            "✗ 未找到统一远程仓库配置 '{}'，请先运行 apply <仓库名>",
            REMOTE_NAME
        );
        return Ok(());
    }

    let branch = current_branch()?;
    let urls = run_git_get_push_urls(REMOTE_NAME)?;

    if urls.is_empty() {
        println!("✗ 远程仓库 '{}' 未配置推送地址", REMOTE_NAME);
        return Ok(());
    }

    // 准备配置的 remote 列表，按 base 长度降序排序，以确保最长前缀匹配
    let mut config_remotes = config.remotes;
    config_remotes.sort_by(|a, b| b.base.len().cmp(&a.base.len()));

    let mut success_count = 0;
    let mut fail_count = 0;

    for url in urls {
        // 匹配 URL 对应的配置名称
        let mut matched_name = None;
        for remote in &config_remotes {
            // 检查 base 是否是 url 的前缀，并且确保边界正确（避免 example.com/foo 匹配 example.com/foobar）
            if url.starts_with(&remote.base) {
                let remainder = &url[remote.base.len()..];
                if remainder.is_empty()
                    || remote.base.ends_with('/')
                    || remote.base.ends_with(':')
                    || remainder.starts_with('/')
                    || remainder.starts_with(':')
                {
                    matched_name = Some(remote.name.clone());
                    break;
                }
            }
        }

        let display_name = matched_name.clone().unwrap_or_else(|| "未命名".to_string());

        // 过滤逻辑
        if !only.is_empty() {
            if let Some(name) = &matched_name {
                if !only.contains(name) {
                    continue;
                }
            } else {
                // 如果指定了 only，未命名的仓库默认跳过
                continue;
            }
        }

        if !except.is_empty() {
            if let Some(name) = &matched_name {
                if except.contains(name) {
                    continue;
                }
            }
        }

        // 在推送前检查远程仓库可用性
        if !dry_run {
            print!(
                "检查远程仓库 '{}' ({}) 的可用性...",
                display_name, url
            );
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
            // 显示完整命令，包含额外参数
            let mut cmd_parts = vec!["git", "push"];
            if options.force {
                cmd_parts.push("--force");
            }
            if options.force_with_lease {
                cmd_parts.push("--force-with-lease");
            }
            if options.set_upstream {
                cmd_parts.push("--set-upstream");
            }
            if options.tags {
                cmd_parts.push("--tags");
            }
            cmd_parts.push(&url);
            cmd_parts.push(&branch);
            for arg in &options.extra_args {
                cmd_parts.push(arg);
            }
            println!("{}", cmd_parts.join(" "));
        } else {
            match run_git_push(&url, &branch, options) {
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
