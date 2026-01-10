use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "yewpb",
    version,
    about = "保存多个 git 远程地址，并应用到当前仓库"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 配置管理（远程仓库的增删改查、导入导出）
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    /// 将已保存的远程仓库应用到当前 git 仓库
    Apply {
        repo: Option<String>,
        /// 自动确认推断的仓库名称
        #[arg(short = 'y', long = "yes")]
        yes: bool,
        /// 连接检查超时时间（秒）
        #[arg(long = "timeout")]
        timeout: Option<u64>,
        /// 仅显示将要执行的操作，不实际修改
        #[arg(short = 'd', long = "dry-run")]
        dry_run: bool,
        /// 不安装 pre-push hook
        #[arg(long = "no-hook")]
        no_hook: bool,
    },
    /// 清理本工具创建的远程仓库
    Clean {
        /// 仅显示将要执行的操作，不实际修改
        #[arg(short = 'd', long = "dry-run")]
        dry_run: bool,
    },
    /// 推送当前分支到所有已配置的远程仓库
    Push {
        #[arg(short = 'd', long = "dry-run")]
        dry_run: bool,
        /// 仅推送到指定名称的仓库（可多次使用）
        #[arg(long = "only")]
        only: Vec<String>,
        /// 排除指定名称的仓库（可多次使用）
        #[arg(long = "except")]
        except: Vec<String>,
        /// 强制推送（覆盖远程历史）
        #[arg(short = 'f', long = "force")]
        force: bool,
        /// 安全的强制推送（推荐替代 --force）
        #[arg(long = "force-with-lease")]
        force_with_lease: bool,
        /// 设置上游跟踪分支
        #[arg(short = 'u', long = "set-upstream")]
        set_upstream: bool,
        /// 同时推送所有标签
        #[arg(long = "tags")]
        tags: bool,
        /// 传递额外的 git 参数，可多次使用（如 --git-args="--no-verify"）
        #[arg(long = "git-args")]
        git_args: Vec<String>,
        /// 推送失败时的最大重试次数
        #[arg(long = "retry")]
        retry: Option<u32>,
        /// 重试间隔毫秒数
        #[arg(long = "retry-delay")]
        retry_delay: Option<u64>,
        /// 跳过连接验证，直接尝试推送
        #[arg(long = "skip-check")]
        skip_check: bool,
        /// 超时时间（秒），0 表示不限制
        #[arg(long = "timeout")]
        timeout: Option<u64>,
    },
    /// 查看各远程仓库的同步状态
    Status,
    /// 检查远程仓库连接是否正常
    Check {
        /// 连接检查超时时间（秒）
        #[arg(long = "timeout")]
        timeout: Option<u64>,
    },
    /// 管理 git alias，将本工具注册为 git 子命令
    Alias {
        /// 别名名称（默认为 "pb" 或配置中的值）
        #[arg(short = 'n', long = "name")]
        name: Option<String>,
        /// 删除别名
        #[arg(short = 'r', long = "remove")]
        remove: bool,
        /// 显示别名状态
        #[arg(short = 's', long = "show")]
        show: bool,
    },
    /// 管理 git pre-push hook
    Hook {
        #[command(subcommand)]
        command: HookCommands,
    },
}

/// 配置相关的子命令
#[derive(Subcommand)]
pub enum ConfigCommands {
    /// 添加或更新远程仓库配置
    Set {
        name: String,
        base: String,
        /// 可选的备注信息，仅作为提示
        #[arg(short = 'n', long = "note")]
        note: Option<String>,
    },
    /// 移除远程仓库配置
    Remove { name: String },
    /// 列出所有远程仓库
    List {
        /// 显示完整详情
        #[arg(short = 'l', long = "long")]
        long: bool,
    },
    /// 导出配置到文件
    Export {
        #[arg(short = 'o', long = "output")]
        output: Option<PathBuf>,
    },
    /// 从文件导入配置
    Import {
        #[arg(short = 'i', long = "input")]
        input: Option<PathBuf>,
        #[arg(short = 'm', long = "merge")]
        merge: bool,
    },
    /// 使用默认编辑器打开配置文件
    Edit,
}

/// Hook 管理子命令
#[derive(Subcommand)]
pub enum HookCommands {
    /// 安装 pre-push hook，推送到 origin 时自动同步到所有远程
    Install {
        /// 自动确认，不询问
        #[arg(short = 'y', long = "yes")]
        yes: bool,
    },
    /// 卸载 pre-push hook
    Uninstall {
        /// 自动确认，不询问
        #[arg(short = 'y', long = "yes")]
        yes: bool,
    },
    /// 查看 hook 安装状态
    Status,
}
