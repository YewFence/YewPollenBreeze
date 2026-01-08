pub fn build_remote_url(base: &str, repo: &str) -> String {
    // 生成完整的远程地址
    // 清理仓库名，统一拼接并确保只有一个 .git 后缀
    let repo = repo.trim();
    let repo = repo.trim_start_matches('/').trim_end_matches('/');
    let repo = repo.strip_suffix(".git").unwrap_or(repo);
    let mut url = if base.ends_with('/') || base.ends_with(':') {
        format!("{base}{repo}")
    } else {
        format!("{base}/{repo}")
    };
    if !url.ends_with(".git") {
        url.push_str(".git");
    }
    url
}
