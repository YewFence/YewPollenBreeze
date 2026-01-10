mod export;
mod import_cmd;
mod list;
mod remove;
mod set;
mod show;

use crate::cli::ConfigCommands;
use anyhow::Result;
use std::path::Path;

/// 配置子命令的统一分发入口
pub fn execute(config_path: &Path, cmd: ConfigCommands) -> Result<()> {
    match cmd {
        ConfigCommands::Set { name, base, note } => set::execute(config_path, name, base, note),
        ConfigCommands::Remove { name } => remove::execute(config_path, name),
        ConfigCommands::List => list::execute(config_path),
        ConfigCommands::Show { name } => show::execute(config_path, name),
        ConfigCommands::Export { output } => export::execute(config_path, output),
        ConfigCommands::Import { input, merge } => import_cmd::execute(config_path, input, merge),
    }
}
