use anyhow::{bail, Context, Result};
use std::collections::HashSet;
use std::path::Path;
use std::process::Command;

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

pub fn check_remote_available(remote_name: &str) -> Result<bool> {
    // 检查远程仓库是否可访问
    // 使用 git ls-remote 命令测试连接性
    let output = Command::new("git")
        .args(["ls-remote", remote_name])
        .output()
        .with_context(|| format!("无法检查远程仓库 '{}' 的可用性", remote_name))?;

    Ok(output.status.success())
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

pub fn run_git_get_push_urls(name: &str) -> Result<Vec<String>> {
    let output = run_git_capture(&["remote", "get-url", "--all", "--push", name])?;
    Ok(output.lines().map(String::from).collect())
}

pub fn run_git_push(remote: &str, branch: &str) -> Result<()> {
    run_git(&["push", remote, branch])
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
