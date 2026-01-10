use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "push-backup",
    version,
    about = "保存多个 git 远程地址，并应用到当前仓库"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 在本地配置中新增或更新远程仓库基础地址
    Add {
        name: String,
        base: String,
        /// 可选的备注信息，仅作为提示
        #[arg(short = 'n', long = "note")]
        note: Option<String>,
    },
    /// 从本地配置中移除远程仓库
    Remove { name: String },
    /// 列出已保存的远程仓库
    List,
    /// 显示远程仓库详情（不指定名称则显示全部）
    Show { name: Option<String> },
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
    /// 导出配置到文件
    Export {
        #[arg(short = 'o', long = "output")]
        output: Option<PathBuf>,
    },
    /// 从文件导入配置
    Import {
        #[arg(short = 'i', long = "input")]
        input: PathBuf,
        #[arg(short = 'm', long = "merge")]
        merge: bool,
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
}
