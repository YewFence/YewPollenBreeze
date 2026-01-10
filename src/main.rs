mod cli;
mod commands;
mod config;
mod git;
mod utils;

use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;
use git::{PushOptions, RetryConfig};

fn main() -> Result<()> {
    // 加载 .env，便于本地开发配置环境变量
    let _ = dotenv();
    // 命令入口，负责分发子命令并执行核心逻辑
    let cli = cli::Cli::parse();
    let config_path = config::config_path()?;

    match cli.command {
        cli::Commands::Add { name, base, note } => commands::add(&config_path, name, base, note),
        cli::Commands::Remove { name } => commands::remove(&config_path, name),
        cli::Commands::List => commands::list(&config_path),
        cli::Commands::Show { name } => commands::show(&config_path, name),
        cli::Commands::Apply { repo, yes } => commands::apply(&config_path, repo, yes),
        cli::Commands::Clean => commands::clean(),
        cli::Commands::Push {
            dry_run,
            only,
            except,
            force,
            force_with_lease,
            set_upstream,
            tags,
            git_args,
            retry,
            retry_delay,
            skip_check,
            timeout,
        } => {
            // 使用 shlex 解析每个 git_args，支持引号包裹的参数
            let extra_args: Vec<String> = git_args
                .iter()
                .flat_map(|s| shlex::split(s).unwrap_or_else(|| vec![s.clone()]))
                .collect();

            let options = PushOptions {
                force,
                force_with_lease,
                set_upstream,
                tags,
                extra_args,
            };

            let retry_config = RetryConfig {
                max_retries: retry,
                delay_ms: retry_delay,
                timeout_secs: timeout,
            };

            commands::push(&config_path, dry_run, only, except, &options, &retry_config, skip_check)
        }
        cli::Commands::Export { output } => commands::export(&config_path, output),
        cli::Commands::Import { input, merge } => commands::import(&config_path, input, merge),
        cli::Commands::Status => commands::status(&config_path),
        cli::Commands::Check => commands::check(&config_path),
    }
}
