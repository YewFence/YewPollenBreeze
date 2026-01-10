mod cli;
mod commands;
mod config;
mod git;
mod utils;

use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;

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
        } => commands::push(&config_path, dry_run, only, except),
        cli::Commands::Export { output } => commands::export(&config_path, output),
        cli::Commands::Import { input, merge } => commands::import(&config_path, input, merge),
    }
}
