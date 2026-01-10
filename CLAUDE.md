# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

yewpb (YewPollenBreeze) 是一个 Rust CLI 工具，用于管理多个 git 远程仓库。它可以保存多个 git 托管平台的基础 URL，然后一键配置当前仓库的 remotes，并将当前分支推送到所有已配置的远程仓库。

## 常用命令

```bash
# 构建项目
cargo build

# 构建发布版本
cargo build --release

# 运行（开发模式）
cargo run -- <subcommand>

# 检查代码
cargo check

# 格式化代码
cargo fmt

# 代码静态检查
cargo clippy
```

## 开发环境配置

设置 `YEWPB_ENV=dev` 可启用开发模式，配置文件将保存在项目根目录的 `.dev/config.toml`，而非系统配置目录。可在项目根目录创建 `.env` 文件：

```bash
YEWPB_ENV=dev
```

## 代码架构

```
src/
├── main.rs      # 入口点，加载 .env，解析 CLI 参数并分发到子命令
├── cli.rs       # 使用 clap derive 定义 CLI 结构和所有子命令
├── config.rs    # 配置文件管理（Config/Remote 结构体，load/save 函数）
├── git.rs       # Git 命令封装（检查仓库、获取分支、远程操作等）
├── utils.rs     # 通用工具函数
└── commands/    # 子命令实现
    ├── mod.rs   # 导出所有子命令的 execute 函数
    ├── add.rs   # 添加远程仓库配置
    ├── remove.rs # 移除远程仓库配置
    ├── list.rs  # 列出已保存的远程仓库
    ├── show.rs  # 显示远程仓库详情
    ├── apply.rs # 将配置应用到当前 git 仓库
    ├── clean.rs # 清理本工具创建的远程仓库
    ├── push.rs  # 推送到所有远程仓库
    ├── status.rs # 查看同步状态
    ├── check.rs # 检查远程连接
    ├── export.rs # 导出配置
    └── import_cmd.rs # 导入配置
```

### 关键设计

- **统一远程命名策略**：所有由本工具创建的 git remote 使用 `yewpb` 命名，便于识别和清理
- **配置文件格式**：使用 TOML 格式存储 `Config { remotes: Vec<Remote> }`，每个 Remote 包含 name、base（基础 URL）、note（可选备注）
- **命令模式**：每个子命令在 `commands/` 目录下有独立文件，导出 `execute` 函数，由 `main.rs` 统一调度

### codeagent-wrapper 使用说明

如果 codeagent-wrapper 调用错误，请带上路径：

```bash
which codeagent-wrapper
# ~/.claude/bin/codeagent-wrapper
```

```
codeagent-wrapper - Go wrapper for AI CLI backends

Usage:
    codeagent-wrapper "task" [workdir]
    codeagent-wrapper --backend codex "task" [workdir]
    codeagent-wrapper - [workdir]              Read task from stdin
    codeagent-wrapper resume <session_id> "task" [workdir]
    codeagent-wrapper resume <session_id> - [workdir]
    codeagent-wrapper --parallel               Run tasks in parallel (config from stdin)
    codeagent-wrapper --version
    codeagent-wrapper --help

Parallel mode examples:
    codeagent-wrapper --parallel < tasks.txt
    echo '...' | codeagent-wrapper --parallel
    codeagent-wrapper --parallel <<'EOF'

Environment Variables:
    CODEX_TIMEOUT  Timeout in milliseconds (default: 7200000)

Exit Codes:
    0    Success
    1    General error (missing args, no output)
    124  Timeout
    127  backend command not found
    130  Interrupted (Ctrl+C)
    *    Passthrough from backend process
```
