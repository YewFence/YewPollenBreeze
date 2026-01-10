## 新功能
- [x] 选择性推送 (--only, --except)
- [x] 推送失败重试机制
- [x] 状态查询 (status 命令查看各 remote 的同步状态，check 确认remote 连接)
- [x] 自动检测仓库名 (从已存在的 git remote 或目录名推断)
- [x] 配置导入/导出功能 (export/import 命令已实现，支持覆盖和合并模式)
- [x] 配置管理功能(完整的 CRUD) (add/remove/list/show 命令已实现)
- [x] 清理功能 (clean 命令清理本工具添加的 remote)
- [] 单项目 remote 管理
- [] Github Action 编译发布
- [x] git alias 加载
- [x] githook 支持
- [] lefthook 支持
- [] DEBUG模式

## 已有功能完善
- [x] http 支持 (已实现，build_remote_url 支持任意格式)
- [x] apply/clean : dry run
- [x] 删除 remote (remove 命令已实现)
- [x] 支持其他 git 参数传入 (主要是-f)
- [] 完善 README