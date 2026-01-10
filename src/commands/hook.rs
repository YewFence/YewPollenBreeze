use crate::git::{
    check_git_available, ensure_git_repo, get_pre_push_hook_path, has_pre_push_hook,
    install_pre_push_hook, is_push_backup_hook_installed, uninstall_pre_push_hook,
};
use anyhow::Result;
use std::io::{self, Write};

/// 安装 pre-push hook
pub fn execute_install(yes: bool) -> Result<()> {
    check_git_available()?;
    ensure_git_repo()?;

    let already_installed = is_push_backup_hook_installed()?;
    let has_hook = has_pre_push_hook()?;

    if already_installed {
        println!("pre-push hook 已安装");
        if !yes {
            print!("是否重新安装? (y/n) ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                println!("操作已取消。");
                return Ok(());
            }
        }
    } else if has_hook {
        println!("检测到已存在 pre-push hook");
        println!("push-backup hook 将被追加到现有 hook 之后");
        if !yes {
            print!("是否继续? (y/n) ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                println!("操作已取消。");
                return Ok(());
            }
        }
    }

    install_pre_push_hook()?;

    let hook_path = get_pre_push_hook_path()?;
    println!("pre-push hook 已成功安装");
    println!("位置: {}", hook_path.display());
    println!();
    println!("现在推送到 origin 时会自动同步到所有配置的远程仓库");
    println!("提示: 可通过环境变量临时禁用: PUSH_BACKUP_SKIP_HOOK=1 git push");

    Ok(())
}

/// 卸载 pre-push hook
pub fn execute_uninstall(yes: bool) -> Result<()> {
    check_git_available()?;
    ensure_git_repo()?;

    if !is_push_backup_hook_installed()? {
        println!("pre-push hook 未安装");
        return Ok(());
    }

    if !yes {
        print!("确认卸载 pre-push hook? (y/n) ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("操作已取消。");
            return Ok(());
        }
    }

    uninstall_pre_push_hook()?;
    println!("pre-push hook 已成功卸载");

    Ok(())
}

/// 查看 hook 安装状态
pub fn execute_status() -> Result<()> {
    check_git_available()?;
    ensure_git_repo()?;

    let hook_path = get_pre_push_hook_path()?;

    if !has_pre_push_hook()? {
        println!("状态: pre-push hook 未安装");
        println!("位置: {}", hook_path.display());
        println!();
        println!("运行 'push-backup hook install' 安装 hook");
    } else if is_push_backup_hook_installed()? {
        println!("状态: push-backup hook 已安装");
        println!("位置: {}", hook_path.display());
        println!();
        println!("推送到 origin 时会自动同步到所有远程仓库");
        println!("可通过环境变量临时禁用: PUSH_BACKUP_SKIP_HOOK=1 git push");
    } else {
        println!("状态: pre-push hook 存在但不是由本工具安装");
        println!("位置: {}", hook_path.display());
        println!();
        println!("运行 'push-backup hook install' 将追加 push-backup hook 到现有 hook");
    }

    Ok(())
}
