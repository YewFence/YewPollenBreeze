use crate::config::load_config;
use crate::git::{
    check_git_available, check_remote_available, ensure_git_repo, git_remote_names,
    run_git_add_remote, run_git_set_url,
};
use crate::utils::build_remote_url;
use anyhow::Result;
use std::path::Path;

pub fn execute(config_path: &Path, repo: String) -> Result<()> {
    check_git_available()?;
    let config = load_config(config_path)?;
    if config.remotes.is_empty() {
        println!("没有保存的远程仓库配置。");
        return Ok(());
    }
    ensure_git_repo()?;
    let mut existing = git_remote_names()?;

    for remote in config.remotes {
        let url = build_remote_url(&remote.base, &repo);
        if existing.contains(&remote.name) {
            run_git_set_url(&remote.name, &url)?;
            println!("已更新远程仓库: {}", remote.name);
        } else {
            run_git_add_remote(&remote.name, &url)?;
            existing.insert(remote.name.clone());
            println!("已添加远程仓库: {}", remote.name);
        }

        // 检查远程仓库可用性
        print!("检查远程仓库 '{}' 的可用性...", remote.name);
        match check_remote_available(&remote.name) {
            Ok(true) => println!(" ✓ 可访问"),
            Ok(false) => println!(" ✗ 无法访问（可能需要配置认证或网络不通）"),
            Err(e) => println!(" ✗ 检查失败: {}", e),
        }
    }
    Ok(())
}
