# push-backup

一个小巧的 Rust CLI：保存多个 git 托管平台的基础 URL，然后一键把当前仓库的 remotes 配好，还能把当前分支一次性推到所有 remote。

## 功能

- 保存多个平台的基础 URL（例如 GitHub、Gitee）
- 按仓库名自动拼接完整 URL（自动补 `.git`）
- 一键把所有 remote 写入当前仓库
- 一键把当前分支推送到所有已配置的 remote（支持 dry-run）

## 使用方式

### 1) 保存平台基础 URL

```bash
cargo run -- add github git@github.com:your-name
cargo run -- add gitee git@gitee.com:your-name
```

### 2) 查看已保存的 remote

```bash
cargo run -- list
```

### 3) 把 remotes 应用到当前仓库

```bash
cargo run -- apply repo-name
```

比如当前仓库名是 `demo`，以上配置会生成：

```
git@github.com:your-name/demo.git
git@gitee.com:your-name/demo.git
```

### 4) 推送当前分支到所有 remote

```bash
cargo run -- push
```

只看执行计划（不真正推送）：

```bash
cargo run -- push -d
```

## 配置文件

配置会保存到系统推荐的用户配置目录，文件名为：

```
config.toml
```

在 Windows 下通常是：

```
C:\Users\<你的用户名>\AppData\Roaming\push-backup\push-backup\config.toml
```

## 环境变量与 .env

支持从当前目录的 `.env` 加载环境变量，示例见 `.env.example`。

```bash
PUSH_BACKUP_ENV=dev
```

## 常见提示

- `apply` 需要在 git 仓库目录下运行
- `push` 会读取当前分支，如果是 detached HEAD 会报错提示
