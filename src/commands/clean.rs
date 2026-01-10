use crate::git::{check_git_available, ensure_git_repo, git_remote_names, run_git_remote_remove};
use anyhow::Result;

const REMOTE_NAME: &str = "yewpb";

pub fn execute(dry_run: bool) -> Result<()> {
    check_git_available()?;
    ensure_git_repo()?;

    let existing = git_remote_names()?;

    if existing.contains(REMOTE_NAME) {
        if dry_run {
            println!("[dry-run] 将执行: git remote remove {}", REMOTE_NAME);
        } else {
            run_git_remote_remove(REMOTE_NAME)?;
            println!("已移除远程仓库: {}", REMOTE_NAME);
        }
    } else {
        println!("未找到远程仓库: {}", REMOTE_NAME);
    }

    Ok(())
}
