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
    Apply { repo: String },
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
}
