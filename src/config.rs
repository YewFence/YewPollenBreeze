use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub remotes: Vec<Remote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Remote {
    pub name: String,
    #[serde(alias = "url")]
    pub base: String,
    pub note: Option<String>,
}

pub fn config_path() -> Result<PathBuf> {
    // 计算配置文件路径
    // 开发环境下将配置存到项目根目录的 .dev 文件夹
    if std::env::var("PUSH_BACKUP_ENV")
        .map(|value| value.eq_ignore_ascii_case("dev"))
        .unwrap_or(false)
    {
        let current_dir = std::env::current_dir().context("获取当前目录失败")?;
        return Ok(current_dir.join(".dev").join("config.toml"));
    }

    // 使用系统推荐的配置目录，避免污染项目仓库
    let project_dirs =
        ProjectDirs::from("com", "push-backup", "push-backup").context("获取配置目录失败")?;
    Ok(project_dirs.config_dir().join("config.toml"))
}

pub fn load_config(path: &Path) -> Result<Config> {
    // 读取本地配置，不存在时返回空配置
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = fs::read_to_string(path)
        .with_context(|| format!("读取配置文件失败: {}", path.display()))?;
    let config = toml::from_str(&content)
        .with_context(|| format!("配置文件格式不合法: {}", path.display()))?;
    Ok(config)
}

pub fn save_config(path: &Path, config: &Config) -> Result<()> {
    // 保存配置到本地文件
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("创建配置目录失败: {}", parent.display()))?;
    }
    let content = toml::to_string_pretty(config).context("序列化配置失败")?;
    fs::write(path, content).with_context(|| format!("写入配置文件失败: {}", path.display()))?;
    Ok(())
}
