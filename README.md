# YewPollenBreeze (yewpb)

**YewPollenBreeze** 是一个 Git 多远程仓库管理与推送工具。它能帮助你轻松管理多个代码托管平台（如 GitHub、Gitee 等）的远程地址，支持一键配置、并发推送、状态检查以及自动重试等功能。

## ✨ 主要功能

- **多平台管理**：集中管理多个 Git 平台的 URL 模板。
- **智能配置**：一键将所有配置的远程仓库应用到当前项目（`apply`），支持自动推断仓库名。
- **并发推送**：多线程并发推送到所有远程仓库，提高推送速度，并附带可视化进度条。
- **高可用性**：支持连接检查、自动重试机制、超时控制，确保网络波动时的推送成功率。
- **灵活控制**：支持 `--only`/`--except` 过滤特定仓库，支持 `dry-run` 预览即将运行的 `git` 命令。
- **Git 集成**：提供 Git Alias (`git pb`) 和 Pre-push Hook 支持，确保使用便捷。
- **状态感知**：提供 `check` 和 `status` 命令，随时掌握远程仓库的连接与同步状态。
- **配置导入/导出**：方便在不同机器间迁移配置。

## 🚀 快速开始

### 1. 安装

您可以直接从 [GitHub Releases](https://github.com/YewFence/YewPollenBreeze/releases/latest) 页面下载最新版本的二进制文件。

下载后，请将可执行文件重命名为 `yewpb` (Windows下为 `yewpb.exe`) 并放入系统的 PATH 路径中。

### 2. 配置远程平台模板 URL

设置各个平台的 URL 基础格式：

```bash
# 设置 GitHub
yewpb config set github git@github.com:your-name

# 设置 Gitea
yewpb config set gitea git@gitea.com:your-name
```

### 3. 应用到当前仓库

在你的 Git 项目根目录下运行：

```bash
# 自动检测当前目录名作为仓库名并应用
yewpb apply

# 或者手动指定仓库名
yewpb apply my-repo-name
```

### 4. 一键推送

```bash
yewpb push
```

工具将并发推送到所有配置的 remote，并显示进度。  
如果出现网络问题，会自动短暂延迟后重试  
相较于原生 `git push` yewpb 的超时时间会更短，并且会有明确的提示
> 主要是我网不好的时候 `git push` 会卡住而且没有任何提示，然后一路等到 TCP 连接超时，有点奇怪，设计如此吗？

## 🛠️ 从源码编译

你也可以尝试从源码编译。

### 前置要求

- [Rust Toolchain](https://rustup.rs/) (1.75.0+)

### 编译步骤

```bash
git clone https://github.com/YewFence/YewPollenBreeze.git
cd YewPollenBreeze
cargo build --release
```

编译完成后，可执行文件位于 `target/release/yewpb` (Windows下为 `yewpb.exe`)。

## 📖 详细使用指南/具体说明

### 配置管理 (`config`)

管理你的远程仓库模板。

- **添加/修改配置**：
  ```bash
  yewpb config set <name> <url>
  # 例：yewpb config set gl git@gitlab.com:user
  ```
- **列出配置**：
  ```bash
  yewpb config list
  ```
- **移除配置**：
  ```bash
  yewpb config remove <name>
  ```
- **导入/导出**：
  ```bash
  yewpb config export > backup.toml
  yewpb config import backup.toml
  ```
- **手动编辑**
  ```bash
  yewpb config edit
  ```
  该命令会使用默认的文本编辑器打开配置文件，你可以手动编辑配置

> 重试相关的逻辑也是可以配置的，请参考 [示例配置文件 `yewpb.example.toml`](./yewpb.example.toml)，然后使用 `yewpb config edit` 命令编辑配置文件

### 仓库设置 (`apply` / `clean`)

- **应用配置**：
  ```bash
  yewpb apply [repo_name] [--dry-run]
  ```
  不指定 `repo_name` 时会自动尝试从目录名推断。
  > 这将自动生成一个 `git remote` 并添加到当前 git 配置中，该 `remote` 中有配置文件中的所有 url 并自动拼接你设定的仓库名。  
  > 同时，它会询问你是否自动添加一个 `git hook` ，作用是在你手动推送 `origin` 时同时运行 `yewpb` 自动备份

- **清理配置**：
  ```bash
  yewpb clean
  ```
  移除由 yewpb 添加的 remote。

### 推送操作 (`push`)

并发推送命令。

```bash
yewpb push [参数]
```

**常用参数：**
- `--dry-run` (`-d`)：仅打印计划，不实际推送。
- `--only <name>`：仅推送到指定仓库（可多次使用）。
- `--except <name>`：排除指定仓库。
- `--force` / `--force-with-lease`：强制推送支持。
- `--git-args`：透传参数给 git push。

> 该命令会手动指定 url 推送，所以不会触发 `pre-push hook`
 
### 状态检查 (`check` / `status`)

- **连接检查**：
  ```bash
  yewpb check
  ```
  测试所有远程仓库的网络连通性。

> 使用 `git ls-remote`

- **同步状态**：
  ```bash
  yewpb status
  ```
  查看当前分支与各远程分支的差异（领先/落后提交数）。

### Git 集成 (`alias` / `hook`)

- **注册 Git Alias**：
  ```bash
  yewpb alias install
  ```
  之后可以使用 `git pb` 运行该程序。

- **卸载 Git Alias**
  ```bash
  yewpb alias --remove
  ```

- **Pre-push Hook**：
  ```bash
  yewpb hook install
  ```
  安装后，执行标准的 `git push origin ...` 时，会自动触发 yewpb 将代码同步到其他所有镜像仓库。

> 需要注意的是，安装/删除该 `hook` 不会覆盖原自定义的 `hook` ，但是如果 `hook` 文件开头没有 shebang 语句它会自动加上 

> ### 完整使用指南：
> 请参考[USAGE.md](./USAGE.md)  
> 该文件由 `clap` 框架自动生成，应该是没啥问题的（大概

## ⚙️ 配置文件

配置文件默认位置：
- Windows: `C:\Users\<User>\AppData\Roaming\yewfence\yewpb\config.toml`
- Linux: `~/.config/yewpb/config.toml`
- macOS: `~/Library/Application Support/yewpb/config.toml`

### 环境变量

- `YEWPB_ENV=dev`：开发模式，配置文件将读取项目根目录下的 `.dev/config.toml`，而不是持久化到大老远的配置目录。
- 支持读取当前目录下的 `.env` 文件。

## 😶‍🌫️ 碎碎念
> 这个名字也是贯彻了我一贯的风格，实际上原来没想好的时候是用的 Push Backup，现在的首字母仍然是 pb ，也是呼应了  
> Pollen: 花粉 Breeze: 微风，意指一段代码如同飘飞的花粉一般乘风上云端
