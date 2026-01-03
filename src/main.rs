use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(
    name = "push-backup",
    version,
    about = "Persist multiple git remote URLs and apply them to the current repo"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add or update a remote base URL in the local config
    Add { name: String, base: String },
    /// Remove a remote from the local config
    Remove { name: String },
    /// List saved remotes
    List,
    /// Apply saved remotes to the current git repository
    Apply { repo: String },
    /// Push current branch to all configured remotes
    Push {
        #[arg(short = 'd', long = "dry-run")]
        dry_run: bool,
    },
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Config {
    remotes: Vec<Remote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Remote {
    name: String,
    #[serde(alias = "url")]
    base: String,
}

fn main() -> Result<()> {
    // 命令入口，负责分发子命令并执行核心逻辑
    let cli = Cli::parse();
    let config_path = config_path()?;

    match cli.command {
        Commands::Add { name, base } => {
            let mut config = load_config(&config_path)?;
            let mut updated = false;
            for remote in &mut config.remotes {
                if remote.name == name {
                    remote.base = base.clone();
                    updated = true;
                    break;
                }
            }
            if !updated {
                config.remotes.push(Remote { name, base });
            }
            save_config(&config_path, &config)?;
            println!("Saved.");
        }
        Commands::Remove { name } => {
            let mut config = load_config(&config_path)?;
            let before = config.remotes.len();
            config.remotes.retain(|remote| remote.name != name);
            if config.remotes.len() == before {
                println!("No matching remote found.");
                return Ok(());
            }
            save_config(&config_path, &config)?;
            println!("Removed.");
        }
        Commands::List => {
            let config = load_config(&config_path)?;
            if config.remotes.is_empty() {
                println!("No remotes saved.");
                return Ok(());
            }
            for remote in config.remotes {
                println!("{}\t{}", remote.name, remote.base);
            }
        }
        Commands::Apply { repo } => {
            let config = load_config(&config_path)?;
            if config.remotes.is_empty() {
                println!("No remotes saved.");
                return Ok(());
            }
            ensure_git_repo()?;
            let mut existing = git_remote_names()?;
            for remote in config.remotes {
                let url = build_remote_url(&remote.base, &repo);
                if existing.contains(&remote.name) {
                    run_git(&["remote", "set-url", &remote.name, &url])?;
                    println!("Updated remote: {}", remote.name);
                } else {
                    run_git(&["remote", "add", &remote.name, &url])?;
                    existing.insert(remote.name.clone());
                    println!("Added remote: {}", remote.name);
                }
            }
        }
        Commands::Push { dry_run } => {
            let config = load_config(&config_path)?;
            if config.remotes.is_empty() {
                println!("No remotes saved.");
                return Ok(());
            }
            ensure_git_repo()?;
            let existing = git_remote_names()?;
            let branch = current_branch()?;
            for remote in config.remotes {
                if !existing.contains(&remote.name) {
                    bail!(
                        "Remote '{}' not found in this repo. Run apply <repo> first.",
                        remote.name
                    );
                }
                if dry_run {
                    println!("git push {} {}", remote.name, branch);
                } else {
                    run_git(&["push", &remote.name, &branch])?;
                    println!("Pushed to remote: {}", remote.name);
                }
            }
        }
    }

    Ok(())
}

fn config_path() -> Result<PathBuf> {
    // 计算配置文件路径
    // 使用系统推荐的配置目录，避免污染项目仓库
    let project_dirs = ProjectDirs::from("com", "push-backup", "push-backup")
        .context("Failed to resolve config directory")?;
    Ok(project_dirs.config_dir().join("config.toml"))
}

fn load_config(path: &Path) -> Result<Config> {
    // 读取本地配置，不存在时返回空配置
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config: {}", path.display()))?;
    let config = toml::from_str(&content)
        .with_context(|| format!("Invalid config format: {}", path.display()))?;
    Ok(config)
}

fn save_config(path: &Path, config: &Config) -> Result<()> {
    // 保存配置到本地文件
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create config dir: {}", parent.display()))?;
    }
    let content = toml::to_string_pretty(config).context("Failed to serialize config")?;
    fs::write(path, content)
        .with_context(|| format!("Failed to write config: {}", path.display()))?;
    Ok(())
}

fn ensure_git_repo() -> Result<()> {
    // 确认当前目录是可用的 git 仓库
    if !Path::new(".git").exists() {
        bail!("Current directory is not a git repository.");
    }
    run_git(&["rev-parse", "--git-dir"])?;
    Ok(())
}

fn git_remote_names() -> Result<HashSet<String>> {
    // 获取当前仓库已有的 remote 名称集合
    let output = run_git_capture(&["remote"])?;
    let names = output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect::<HashSet<_>>();
    Ok(names)
}

fn build_remote_url(base: &str, repo: &str) -> String {
    // 生成完整 remote URL
    // 清理仓库名，统一拼接并确保只有一个 .git 后缀
    let repo = repo.trim();
    let repo = repo.trim_start_matches('/').trim_end_matches('/');
    let repo = repo.strip_suffix(".git").unwrap_or(repo);
    let mut url = if base.ends_with('/') || base.ends_with(':') {
        format!("{base}{repo}")
    } else {
        format!("{base}/{repo}")
    };
    if !url.ends_with(".git") {
        url.push_str(".git");
    }
    url
}

fn run_git(args: &[&str]) -> Result<()> {
    // 执行 git 命令，不关心输出
    let output = Command::new("git")
        .args(args)
        .output()
        .with_context(|| format!("Failed to run git {}", args.join(" ")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Git command failed: {}", stderr.trim());
    }
    Ok(())
}

fn run_git_capture(args: &[&str]) -> Result<String> {
    // 执行 git 命令并返回输出内容
    let output = Command::new("git")
        .args(args)
        .output()
        .with_context(|| format!("Failed to run git {}", args.join(" ")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Git command failed: {}", stderr.trim());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn current_branch() -> Result<String> {
    // 获取当前分支名
    // 获取当前分支，避免在 detached HEAD 下误推送
    let branch = run_git_capture(&["rev-parse", "--abbrev-ref", "HEAD"])?;
    if branch == "HEAD" {
        bail!("Detached HEAD; please checkout a branch before pushing.");
    }
    Ok(branch)
}
