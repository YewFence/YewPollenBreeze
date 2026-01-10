use crate::config::{load_config, DEFAULT_ALIAS};
use crate::git::{
    check_git_available, get_git_alias, set_git_alias, unset_git_alias, which_command,
};
use anyhow::Result;
use std::path::Path;

pub fn execute(config_path: &Path, name: Option<String>, remove: bool, show: bool) -> Result<()> {
    check_git_available()?;

    // 确定要使用的别名名称
    let config = load_config(config_path)?;
    let alias_name = name
        .or(config.defaults.alias.clone())
        .unwrap_or_else(|| DEFAULT_ALIAS.to_string());

    // 显示模式
    if show {
        return show_alias_status(&alias_name);
    }

    // 删除模式
    if remove {
        return remove_alias(&alias_name);
    }

    // 安装模式
    install_alias(&alias_name)
}

fn show_alias_status(name: &str) -> Result<()> {
    match get_git_alias(name)? {
        Some(value) => {
            if value.contains("yewpb") {
                println!("别名 'git {}' 已配置，指向: {}", name, value);
            } else {
                println!("别名 'git {}' 已存在，但不是由本工具创建: {}", name, value);
            }
        }
        None => {
            println!("别名 'git {}' 未配置", name);
        }
    }
    Ok(())
}

fn remove_alias(name: &str) -> Result<()> {
    // 检查别名是否存在
    match get_git_alias(name)? {
        Some(value) => {
            // 检查是否是我们创建的别名
            if !value.contains("yewpb") {
                println!("警告: 别名 'git {}' 不是由本工具创建，仍将删除", name);
            }
            unset_git_alias(name)?;
            println!("已删除别名: git {}", name);
        }
        None => {
            println!("别名 'git {}' 不存在", name);
        }
    }
    Ok(())
}

fn install_alias(name: &str) -> Result<()> {
    // 查找 yewpb 的完整路径
    let cmd_path = which_command("yewpb")?;

    // 构建 alias 命令
    // 使用 ! 前缀让 git 把它当作 shell 命令执行
    let alias_value = match cmd_path {
        Some(path) => {
            // Windows 下路径可能包含空格和反斜杠，需要处理
            if cfg!(windows) {
                // Windows: 使用引号包裹路径，反斜杠转为正斜杠
                format!("!\"{}\"", path.replace('\\', "/"))
            } else {
                format!("!{}", path)
            }
        }
        None => {
            // 如果找不到完整路径，直接使用命令名
            // 假设它在 PATH 中
            "!yewpb".to_string()
        }
    };

    // 检查是否已存在
    if let Some(existing) = get_git_alias(name)? {
        if existing == alias_value {
            println!("别名 'git {}' 已是最新配置", name);
            return Ok(());
        }
        println!("更新已存在的别名: git {}", name);
    }

    set_git_alias(name, &alias_value)?;
    println!("已设置别名: git {} -> {}", name, alias_value);
    println!("\n现在可以使用 'git {}' 来运行 yewpb", name);

    Ok(())
}
