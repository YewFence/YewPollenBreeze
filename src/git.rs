use anyhow::{bail, Context, Result};
use std::collections::HashSet;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;

pub fn check_git_available() -> Result<()> {
    // 检查 git 命令是否可用
    Command::new("git")
        .arg("--version")
        .output()
        .context("Git 命令不可用，请确保已安装 Git 并添加到 PATH 环境变量")?;
    Ok(())
}

pub fn ensure_git_repo() -> Result<()> {
    // 确认当前目录是可用的 git 仓库
    if !Path::new(".git").exists() {
        bail!("当前目录不是 git 仓库。");
    }
    run_git(&["rev-parse", "--git-dir"])?;
    Ok(())
}

pub fn git_remote_names() -> Result<HashSet<String>> {
    // 获取当前仓库已有的远程仓库名称集合
    let output = run_git_capture(&["remote"])?;
    let names = output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect::<HashSet<_>>();
    Ok(names)
}

pub fn current_branch() -> Result<String> {
    // 获取当前分支，避免在游离 HEAD 状态下误推送
    let branch = run_git_capture(&["rev-parse", "--abbrev-ref", "HEAD"])?;
    if branch == "HEAD" {
        bail!("当前处于游离 HEAD 状态，请先切换到分支再推送。");
    }
    Ok(branch)
}

pub fn check_remote_available(remote_name: &str, timeout_secs: u64) -> Result<bool> {
    // 检查远程仓库是否可访问，带超时控制
    let mut child = Command::new("git")
        .args(["ls-remote", remote_name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .with_context(|| format!("无法检查远程仓库 '{}' 的可用性", remote_name))?;

    let timeout = Duration::from_secs(timeout_secs);
    match child.wait_timeout(timeout)? {
        Some(status) => Ok(status.success()),
        None => {
            // 超时，杀死进程
            let _ = child.kill();
            let _ = child.wait();
            bail!("检查超时（{}秒）", timeout_secs)
        }
    }
}

// Git 操作辅助函数
pub fn run_git_add_remote(name: &str, url: &str) -> Result<()> {
    run_git(&["remote", "add", name, url])
}

pub fn run_git_remote_remove(name: &str) -> Result<()> {
    run_git(&["remote", "remove", name])
}

pub fn run_git_add_push_url(name: &str, url: &str) -> Result<()> {
    run_git(&["remote", "set-url", "--add", "--push", name, url])
}

pub fn run_git_get_remote_url(name: &str) -> Result<String> {
    run_git_capture(&["remote", "get-url", name])
}

pub fn run_git_get_push_urls(name: &str) -> Result<Vec<String>> {
    let output = run_git_capture(&["remote", "get-url", "--all", "--push", name])?;
    Ok(output.lines().map(String::from).collect())
}

/// 推送选项
#[derive(Default)]
pub struct PushOptions {
    pub force: bool,
    pub force_with_lease: bool,
    pub set_upstream: bool,
    pub tags: bool,
    pub extra_args: Vec<String>,
}

/// 重试配置
#[derive(Clone, Default)]
pub struct RetryConfig {
    /// 最大重试次数
    pub max_retries: u32,
    /// 重试间隔（毫秒）
    pub delay_ms: u64,
    /// 超时时间（秒），0 表示不限制
    pub timeout_secs: u64,
}

pub fn run_git_push(
    remote: &str,
    branch: &str,
    options: &PushOptions,
    timeout_secs: u64,
) -> Result<()> {
    // 执行 git push 操作，可选是否配置超时
    // 构建 git push 命令参数
    let mut args = vec!["push".to_string()];

    // 专用标志
    if options.force {
        args.push("--force".to_string());
    }
    if options.force_with_lease {
        args.push("--force-with-lease".to_string());
    }
    if options.set_upstream {
        args.push("--set-upstream".to_string());
    }
    if options.tags {
        args.push("--tags".to_string());
    }

    // 远程和分支
    args.push(remote.to_string());
    args.push(branch.to_string());

    // 额外参数（放在最后）
    args.extend(options.extra_args.clone());

    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    if timeout_secs > 0 {
        run_git_with_timeout(&args_ref, timeout_secs)
    } else {
        run_git(&args_ref)
    }
}

// 内部函数：执行 git 命令
fn run_git(args: &[&str]) -> Result<()> {
    // 执行 git 命令，不关心输出
    let output = Command::new("git")
        .args(args)
        .output()
        .with_context(|| format!("执行 git 命令失败: {}", args.join(" ")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git 命令执行失败: {}", stderr.trim());
    }
    Ok(())
}

// 内部函数：执行 git 命令，带超时控制
fn run_git_with_timeout(args: &[&str], timeout_secs: u64) -> Result<()> {
    let mut child = Command::new("git")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("执行 git 命令失败: {}", args.join(" ")))?;

    let timeout = Duration::from_secs(timeout_secs);
    match child.wait_timeout(timeout)? {
        Some(status) => {
            if !status.success() {
                let output = child.wait_with_output()?;
                let stderr = String::from_utf8_lossy(&output.stderr);
                bail!("git 命令执行失败: {}", stderr.trim());
            }
            Ok(())
        }
        None => {
            // 超时，杀死进程
            let _ = child.kill();
            let _ = child.wait();
            bail!("命令超时（{}秒）", timeout_secs)
        }
    }
}

// 内部函数：执行 git 命令并返回输出内容
fn run_git_capture(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .with_context(|| format!("执行 git 命令失败: {}", args.join(" ")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git 命令执行失败: {}", stderr.trim());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// 通过 URL 获取远程分支的 commit hash
pub fn git_ls_remote_ref(url: &str, branch: &str) -> Result<Option<String>> {
    let output = Command::new("git")
        .args(["ls-remote", url, &format!("refs/heads/{}", branch)])
        .output()
        .with_context(|| format!("无法获取远程仓库 '{}' 的引用", url))?;

    if !output.status.success() {
        return Ok(None); // 连接失败或分支不存在
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    // 输出格式: "commit_hash\trefs/heads/branch"
    Ok(stdout.split_whitespace().next().map(String::from))
}

/// 计算本地 HEAD 与远程 commit 之间的 ahead/behind 数量
pub fn git_count_ahead_behind(remote_commit: &str) -> Result<(usize, usize)> {
    let output = run_git_capture(&[
        "rev-list",
        "--left-right",
        "--count",
        &format!("HEAD...{}", remote_commit),
    ])?;
    // 输出格式: "ahead\tbehind"
    let parts: Vec<&str> = output.split_whitespace().collect();
    let ahead = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0);
    let behind = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    Ok((ahead, behind))
}

/// 获取 git config 中的 alias 值
pub fn get_git_alias(name: &str) -> Result<Option<String>> {
    let output = Command::new("git")
        .args(["config", "--global", "--get", &format!("alias.{}", name)])
        .output()
        .context("执行 git config 失败")?;

    if output.status.success() {
        let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(Some(value))
    } else {
        // 返回码 1 表示 key 不存在，其他错误需要报告
        if output.status.code() == Some(1) {
            Ok(None)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("获取 git alias 失败: {}", stderr.trim());
        }
    }
}

/// 设置 git alias（覆盖模式）
pub fn set_git_alias(name: &str, command: &str) -> Result<()> {
    run_git(&[
        "config",
        "--global",
        &format!("alias.{}", name),
        command,
    ])
}

/// 删除 git alias
pub fn unset_git_alias(name: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["config", "--global", "--unset", &format!("alias.{}", name)])
        .output()
        .context("执行 git config 失败")?;

    if output.status.success() || output.status.code() == Some(5) {
        // code 5 表示 key 不存在，这也是成功
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("删除 git alias 失败: {}", stderr.trim());
    }
}

/// 查找可执行文件的完整路径
pub fn which_command(cmd: &str) -> Result<Option<String>> {
    let output = Command::new(if cfg!(windows) { "where" } else { "which" })
        .arg(cmd)
        .output()
        .context("查找命令路径失败")?;

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout)
            .lines()
            .next()
            .map(|s| s.trim().to_string());
        Ok(path)
    } else {
        Ok(None)
    }
}

