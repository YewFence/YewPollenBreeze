use crate::config::load_config;
use crate::git::{
    check_git_available, check_remote_available, current_branch, ensure_git_repo, git_remote_names,
    run_git_get_push_urls, run_git_push, PushOptions, RetryConfig,
};
use anyhow::Result;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::Path;
use std::thread;
use std::time::Duration;

const REMOTE_NAME: &str = "yewpb";

/// 单个仓库的推送任务
struct PushTask {
    url: String,
    display_name: String,
    status: PushStatus,
    attempts: u32,
    last_error: Option<String>,
}

#[derive(Clone, PartialEq)]
enum PushStatus {
    Pending, // 待推送
    Success, // 成功
    Failed,  // 失败
}

/// 单次推送的结果
struct PushResult {
    idx: usize,
    success: bool,
    error: Option<String>,
}

pub fn execute(
    config_path: &Path,
    dry_run: bool,
    only: Vec<String>,
    except: Vec<String>,
    options: &PushOptions,
    retry_config: &RetryConfig,
    skip_check: bool,
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

    // dry-run 模式下直接显示命令，不需要重试逻辑
    if dry_run {
        for url in &urls {
            let display_name = match_display_name(url, &config_remotes);

            // 过滤逻辑
            if !should_push(url, &display_name, &only, &except) {
                continue;
            }

            // 显示完整命令
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
            cmd_parts.push(url);
            cmd_parts.push(&branch);
            for arg in &options.extra_args {
                cmd_parts.push(arg);
            }
            println!("[dry-run] 将执行: {}", cmd_parts.join(" "));
        }
        return Ok(());
    }

    // 初始化推送任务列表
    let mut tasks: Vec<PushTask> = urls
        .into_iter()
        .filter_map(|url| {
            let display_name = match_display_name(&url, &config_remotes);
            if should_push(&url, &display_name, &only, &except) {
                Some(PushTask {
                    url,
                    display_name,
                    status: PushStatus::Pending,
                    attempts: 0,
                    last_error: None,
                })
            } else {
                None
            }
        })
        .collect();

    if tasks.is_empty() {
        println!("✗ 没有符合条件的远程仓库需要推送");
        return Ok(());
    }

    // 主推送循环（包含重试）
    let mut round = 0u32;
    loop {
        let pending_tasks: Vec<usize> = tasks
            .iter()
            .enumerate()
            .filter(|(_, t)| t.status != PushStatus::Success)
            .map(|(i, _)| i)
            .collect();

        if pending_tasks.is_empty() {
            break; // 所有任务成功
        }

        // 检查是否需要重试
        if round > 0 {
            if round > retry_config.max_retries {
                break; // 已达到最大重试次数
            }

            println!(
                "\n⏳ 第 {} 次重试，等待 {}ms...",
                round, retry_config.delay_ms
            );
            thread::sleep(Duration::from_millis(retry_config.delay_ms));
            println!();
        }

        // 创建多进度条
        let mp = MultiProgress::new();

        // 总进度条
        let total_style = ProgressStyle::default_bar()
            .template("{msg} [{bar:20.cyan/dim}] {pos}/{len}")
            .unwrap()
            .progress_chars("█░░");
        let total_bar = mp.add(ProgressBar::new(pending_tasks.len() as u64));
        total_bar.set_style(total_style);
        total_bar.set_message("推送中");

        // 单任务进度条样式
        let style = ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {wide_msg}")
            .unwrap();

        // 为每个待处理任务创建进度条
        let bars: Vec<Option<ProgressBar>> = tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                if pending_tasks.contains(&i) {
                    let pb = mp.add(ProgressBar::new_spinner());
                    pb.set_style(style.clone());
                    pb.set_message(format!("○ {} 等待中", task.display_name));
                    Some(pb)
                } else {
                    None
                }
            })
            .collect();

        // 并发推送
        let results: Vec<PushResult> = pending_tasks
            .par_iter()
            .filter_map(|&idx| {
                let task = &tasks[idx];

                // 首轮之后只处理失败的任务
                if round > 0 && task.status == PushStatus::Pending {
                    return None;
                }

                let pb = bars[idx].as_ref().unwrap();

                // 更新状态：推送中
                pb.set_message(format!("◐ {} 推送中...", task.display_name));
                pb.enable_steady_tick(Duration::from_millis(100));

                // 可用性检查（如果需要）
                if !skip_check {
                    pb.set_message(format!("◐ {} 检查可用性...", task.display_name));
                    match check_remote_available(&task.url, retry_config.timeout_secs) {
                        Ok(true) => {}
                        Ok(false) => {
                            pb.finish_with_message(format!("✗ {} 无法访问", task.display_name));
                            total_bar.inc(1);
                            return Some(PushResult {
                                idx,
                                success: false,
                                error: Some("远程仓库无法访问".to_string()),
                            });
                        }
                        Err(e) => {
                            pb.finish_with_message(format!("✗ {} 检查失败", task.display_name));
                            total_bar.inc(1);
                            return Some(PushResult {
                                idx,
                                success: false,
                                error: Some(format!("检查失败: {}", e)),
                            });
                        }
                    }
                    pb.set_message(format!("◐ {} 推送中...", task.display_name));
                }

                // 执行推送
                match run_git_push(&task.url, &branch, options, retry_config.timeout_secs) {
                    Ok(_) => {
                        pb.finish_with_message(format!("✓ {} 完成", task.display_name));
                        total_bar.inc(1);
                        Some(PushResult {
                            idx,
                            success: true,
                            error: None,
                        })
                    }
                    Err(e) => {
                        pb.finish_with_message(format!("✗ {} 失败", task.display_name));
                        total_bar.inc(1);
                        Some(PushResult {
                            idx,
                            success: false,
                            error: Some(e.to_string()),
                        })
                    }
                }
            })
            .collect();

        // 完成总进度条
        total_bar.finish_with_message("推送完成");

        // 更新任务状态
        for result in results {
            tasks[result.idx].attempts += 1;
            if result.success {
                tasks[result.idx].status = PushStatus::Success;
                tasks[result.idx].last_error = None;
            } else {
                tasks[result.idx].status = PushStatus::Failed;
                tasks[result.idx].last_error = result.error;
            }
        }

        round += 1;
    }

    // 输出汇总
    print_summary(&tasks, retry_config.max_retries);

    Ok(())
}

/// 匹配 URL 对应的显示名称
fn match_display_name(url: &str, config_remotes: &[crate::config::Remote]) -> String {
    for remote in config_remotes {
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

/// 判断是否应该推送到该仓库
fn should_push(url: &str, display_name: &str, only: &[String], except: &[String]) -> bool {
    // only 过滤
    if !only.is_empty() && (display_name == "未命名" || !only.contains(&display_name.to_string()))
    {
        return false;
    }

    // except 过滤
    if !except.is_empty() && display_name != "未命名" && except.contains(&display_name.to_string())
    {
        return false;
    }

    // 避免未使用警告
    let _ = url;

    true
}

/// 输出推送汇总
fn print_summary(tasks: &[PushTask], max_retries: u32) {
    let success: Vec<&PushTask> = tasks
        .iter()
        .filter(|t| t.status == PushStatus::Success)
        .collect();
    let failed: Vec<&PushTask> = tasks
        .iter()
        .filter(|t| t.status != PushStatus::Success)
        .collect();

    println!("\n========== 推送汇总 ==========");
    println!("成功: {} 个", success.len());
    println!("失败: {} 个", failed.len());

    // 显示重试成功的仓库
    if max_retries > 0 {
        let retried_success: Vec<&&PushTask> = success.iter().filter(|t| t.attempts > 1).collect();
        if !retried_success.is_empty() {
            println!("\n重试后成功的仓库:");
            for task in retried_success {
                println!("  ✓ {} (尝试 {} 次)", task.display_name, task.attempts);
            }
        }
    }

    // 显示失败的仓库
    if !failed.is_empty() {
        println!("\n失败的仓库:");
        for task in &failed {
            let error_msg = task.last_error.as_deref().unwrap_or("未知错误");
            println!(
                "  ✗ {} (尝试 {} 次): {}",
                task.display_name, task.attempts, error_msg
            );
        }
    }
}
