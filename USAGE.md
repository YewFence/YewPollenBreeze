#  命令行工具 `yewpb` 的完整使用说明

**命令概览:**

- [命令行工具 `yewpb` 的完整使用说明](#命令行工具-yewpb-的完整使用说明)
  - [`yewpb`](#yewpb)
          - [**Subcommands:**](#subcommands)
  - [`yewpb config`](#yewpb-config)
          - [**Subcommands:**](#subcommands-1)
  - [`yewpb config set`](#yewpb-config-set)
          - [**Arguments:**](#arguments)
          - [**Options:**](#options)
  - [`yewpb config remove`](#yewpb-config-remove)
          - [**Arguments:**](#arguments-1)
  - [`yewpb config list`](#yewpb-config-list)
          - [**Options:**](#options-1)
  - [`yewpb config export`](#yewpb-config-export)
          - [**Options:**](#options-2)
  - [`yewpb config import`](#yewpb-config-import)
          - [**Options:**](#options-3)
  - [`yewpb config edit`](#yewpb-config-edit)
  - [`yewpb apply`](#yewpb-apply)
          - [**Arguments:**](#arguments-2)
          - [**Options:**](#options-4)
  - [`yewpb clean`](#yewpb-clean)
          - [**Options:**](#options-5)
  - [`yewpb push`](#yewpb-push)
          - [**Options:**](#options-6)
  - [`yewpb status`](#yewpb-status)
  - [`yewpb check`](#yewpb-check)
          - [**Options:**](#options-7)
  - [`yewpb alias`](#yewpb-alias)
          - [**Options:**](#options-8)
  - [`yewpb hook`](#yewpb-hook)
          - [**Subcommands:**](#subcommands-2)
  - [`yewpb hook install`](#yewpb-hook-install)
          - [**Options:**](#options-9)
  - [`yewpb hook uninstall`](#yewpb-hook-uninstall)
          - [**Options:**](#options-10)
  - [`yewpb hook status`](#yewpb-hook-status)

## `yewpb`

保存多个 git 远程地址，并应用到当前仓库

**Usage:** `yewpb <COMMAND>`

###### **Subcommands:**

* `config` — 配置管理（远程仓库的增删改查、导入导出）
* `apply` — 将已保存的远程仓库应用到当前 git 仓库
* `clean` — 清理本工具创建的远程仓库
* `push` — 推送当前分支到所有已配置的远程仓库
* `status` — 查看各远程仓库的同步状态
* `check` — 检查远程仓库连接是否正常
* `alias` — 管理 git alias，将本工具注册为 git 子命令
* `hook` — 管理 git pre-push hook



## `yewpb config`

配置管理（远程仓库的增删改查、导入导出）

**Usage:** `yewpb config <COMMAND>`

###### **Subcommands:**

* `set` — 添加或更新远程仓库配置
* `remove` — 移除远程仓库配置
* `list` — 列出所有远程仓库
* `export` — 导出配置到文件
* `import` — 从文件导入配置
* `edit` — 使用默认编辑器打开配置文件



## `yewpb config set`

添加或更新远程仓库配置

**Usage:** `yewpb config set [OPTIONS] <NAME> <BASE>`

###### **Arguments:**

* `<NAME>`
* `<BASE>`

###### **Options:**

* `-n`, `--note <NOTE>` — 可选的备注信息，仅作为提示



## `yewpb config remove`

移除远程仓库配置

**Usage:** `yewpb config remove <NAME>`

###### **Arguments:**

* `<NAME>`



## `yewpb config list`

列出所有远程仓库

**Usage:** `yewpb config list [OPTIONS]`

###### **Options:**

* `-l`, `--long` — 显示完整详情



## `yewpb config export`

导出配置到文件

**Usage:** `yewpb config export [OPTIONS]`

###### **Options:**

* `-o`, `--output <OUTPUT>`



## `yewpb config import`

从文件导入配置

**Usage:** `yewpb config import [OPTIONS]`

###### **Options:**

* `-i`, `--input <INPUT>`
* `-m`, `--merge`



## `yewpb config edit`

使用默认编辑器打开配置文件

**Usage:** `yewpb config edit`



## `yewpb apply`

将已保存的远程仓库应用到当前 git 仓库

**Usage:** `yewpb apply [OPTIONS] [REPO]`

###### **Arguments:**

* `<REPO>`

###### **Options:**

* `-y`, `--yes` — 自动确认推断的仓库名称
* `--timeout <TIMEOUT>` — 连接检查超时时间（秒）
* `-d`, `--dry-run` — 仅显示将要执行的操作，不实际修改
* `--no-hook` — 不安装 pre-push hook



## `yewpb clean`

清理本工具创建的远程仓库

**Usage:** `yewpb clean [OPTIONS]`

###### **Options:**

* `-d`, `--dry-run` — 仅显示将要执行的操作，不实际修改



## `yewpb push`

推送当前分支到所有已配置的远程仓库

**Usage:** `yewpb push [OPTIONS]`

###### **Options:**

* `-d`, `--dry-run`
* `--only <ONLY>` — 仅推送到指定名称的仓库（可多次使用）
* `--except <EXCEPT>` — 排除指定名称的仓库（可多次使用）
* `-f`, `--force` — 强制推送（覆盖远程历史）
* `--force-with-lease` — 安全的强制推送（推荐替代 --force）
* `-u`, `--set-upstream` — 设置上游跟踪分支
* `--tags` — 同时推送所有标签
* `--git-args <GIT_ARGS>` — 传递额外的 git 参数，可多次使用（如 --git-args="--no-verify"）
* `--retry <RETRY>` — 推送失败时的最大重试次数
* `--retry-delay <RETRY_DELAY>` — 重试间隔毫秒数
* `--skip-check` — 跳过连接验证，直接尝试推送
* `--timeout <TIMEOUT>` — 超时时间（秒），0 表示不限制



## `yewpb status`

查看各远程仓库的同步状态

**Usage:** `yewpb status`



## `yewpb check`

检查远程仓库连接是否正常

**Usage:** `yewpb check [OPTIONS]`

###### **Options:**

* `--timeout <TIMEOUT>` — 连接检查超时时间（秒）



## `yewpb alias`

管理 git alias，将本工具注册为 git 子命令

**Usage:** `yewpb alias [OPTIONS]`

###### **Options:**

* `-n`, `--name <NAME>` — 别名名称（默认为 "pb" 或配置中的值）
* `-r`, `--remove` — 删除别名
* `-s`, `--show` — 显示别名状态



## `yewpb hook`

管理 git pre-push hook

**Usage:** `yewpb hook <COMMAND>`

###### **Subcommands:**

* `install` — 安装 pre-push hook，推送到 origin 时自动同步到所有远程
* `uninstall` — 卸载 pre-push hook
* `status` — 查看 hook 安装状态



## `yewpb hook install`

安装 pre-push hook，推送到 origin 时自动同步到所有远程

**Usage:** `yewpb hook install [OPTIONS]`

###### **Options:**

* `-y`, `--yes` — 自动确认，不询问



## `yewpb hook uninstall`

卸载 pre-push hook

**Usage:** `yewpb hook uninstall [OPTIONS]`

###### **Options:**

* `-y`, `--yes` — 自动确认，不询问



## `yewpb hook status`

查看 hook 安装状态

**Usage:** `yewpb hook status`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

