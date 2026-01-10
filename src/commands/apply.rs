use crate::config::load_config;
use crate::git::{
    check_git_available, check_remote_available, ensure_git_repo, git_remote_names,
    install_pre_push_hook, is_push_backup_hook_installed, run_git_add_push_url, run_git_add_remote,
    run_git_get_remote_url, run_git_remote_remove,
};
use crate::utils::build_remote_url;
use anyhow::Result;
use std::env;
use std::io::{self, Write};
use std::path::Path;

const REMOTE_NAME: &str = "push-backup";

pub fn execute(
    config_path: &Path,
    repo: Option<String>,
    yes: bool,
    timeout: u64,
    dry_run: bool,
    no_hook: bool,
) -> Result<()> {
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
            if dry_run {
                println!("[dry-run] 将执行: git remote remove {}", remote.name);
            } else {
                run_git_remote_remove(&remote.name)?;
                println!("已清理旧远程仓库: {}", remote.name);
            }
        }
    }

    // 2. 重置 push-backup 远程仓库
    if existing.contains(REMOTE_NAME) {
        if dry_run {
            println!("[dry-run] 将执行: git remote remove {}", REMOTE_NAME);
        } else {
            run_git_remote_remove(REMOTE_NAME)?;
        }
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
        if dry_run {
            println!(
                "[dry-run] 将执行: git remote add {} {}",
                REMOTE_NAME, first_url
            );
        } else {
            run_git_add_remote(REMOTE_NAME, first_url)?;
            println!("已配置统一远程仓库: {}", REMOTE_NAME);
        }
    }

    // 4. 添加所有 push URL 并检查可用性
    for (name, url) in remote_urls {
        if dry_run {
            println!(
                "[dry-run] 将执行: git remote set-url --add --push {} {}",
                REMOTE_NAME, url
            );
        } else {
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
    }

    // Hook 安装逻辑
    if !dry_run {
        let should_install_hook = if no_hook {
            // 明确指定 --no-hook，跳过安装
            false
        } else if yes {
            // -y 模式下默认安装 hook
            true
        } else {
            // 交互式询问（默认 Y）
            println!();
            print!("是否安装 pre-push hook 以自动同步到所有远程? (Y/n) ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            // 默认为 Y，只有明确输入 n/N 才跳过
            !input.eq_ignore_ascii_case("n")
        };

        if should_install_hook {
            println!();
            let already_installed = is_push_backup_hook_installed().unwrap_or(false);
            if already_installed {
                println!("pre-push hook 已存在，跳过安装");
            } else {
                match install_pre_push_hook() {
                    Ok(_) => {
                        println!("pre-push hook 已成功安装");
                        println!("现在推送到 origin 时会自动同步到所有配置的远程仓库");
                    }
                    Err(e) => println!("hook 安装失败: {}", e),
                }
            }
            println!();
            println!("提示: 可通过环境变量临时禁用 hook: PUSH_BACKUP_SKIP_HOOK=1 git push");
            println!("提示: 卸载 hook 请运行: push-backup hook uninstall");
        }
    }

    Ok(())
}
