# 开源社区贡献者激励平台

基于 Rust 的全栈开源社区贡献者激励平台，追踪 GitHub 贡献数据，计算贡献分，展示排行榜。

## 技术栈

- **后端**: Actix-web 4 + SQLite (rusqlite)
- **前端**: Yew 0.21 (WASM) + yew-router
- **数据源**: GitHub REST API (Personal Access Token)
- **部署**: 单二进制文件，Actix 同时托管 API 和前端静态文件

## 功能特性

- 绑定 GitHub 账号（用户名 + PAT）
- 定期自动同步 PR 和 Issue 数据
- 按代码行数和评论数计算贡献分
- 贡献者排行榜（全局 + 按项目）
- 项目热度展示（星标、Fork、Issue 数）
- 维护者手动发放特殊贡献分
- 所有数据本地 SQLite 缓存

## 积分规则

```
总分 = 代码行数分 + 评论分 + 奖励分

代码行数分 = SUM(additions × 1.0 + deletions × 0.5)  // 仅计算已合并的 PR
评论分 = issue评论数 × 2 + PR评论数 × 3 + 创建的issue数 × 5
奖励分 = 维护者手动发放的分数总和
```

## 快速开始

### 前置条件

- Rust 工具链 (rustup)
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- wasm32 target (`rustup target add wasm32-unknown-unknown`)

### 构建前端

```bash
cd frontend
trunk build --release
```

### 运行后端

```bash
cd backend
cargo run
```

服务启动在 `http://127.0.0.1:8080`

### 配置

编辑 `config.toml`:

```toml
database_path = "./data.db"
server_host = "127.0.0.1"
server_port = 8080
sync_interval_seconds = 3600
frontend_dist_path = "./frontend/dist"
github_api_base_url = "https://api.github.com"
```

## API 端点

| 方法 | 路径 | 描述 |
|------|------|------|
| POST | `/api/v1/users` | 绑定 GitHub 账号 |
| GET | `/api/v1/users` | 获取用户列表 |
| POST | `/api/v1/projects` | 添加跟踪项目 |
| GET | `/api/v1/projects` | 获取项目列表 |
| GET | `/api/v1/leaderboard` | 获取排行榜 |
| POST | `/api/v1/bonus` | 发放奖励分 |
| POST | `/api/v1/sync` | 触发手动同步 |
| GET | `/api/v1/sync/status` | 获取同步状态 |
| GET | `/api/v1/contributions/prs?user_id=` | 获取用户 PR 列表 |
| GET | `/api/v1/contributions/issues?user_id=` | 获取用户 Issue 列表 |

## 项目结构

```
├── Cargo.toml              # Workspace 根配置
├── config.toml             # 运行时配置
├── backend/                # Actix-web 后端
│   └── src/
│       ├── main.rs         # 入口：启动服务器 + 后台同步任务
│       ├── api/            # REST API 端点
│       ├── db/             # SQLite 数据库操作
│       ├── github/         # GitHub API 客户端 + 同步逻辑
│       ├── models/         # 数据模型
│       └── scoring/        # 积分计算
├── frontend/               # Yew WASM 前端
│   └── src/
│       ├── app.rs          # 路由 + 根组件
│       ├── api.rs          # HTTP 客户端
│       ├── components/     # UI 组件
│       └── pages/          # 页面
└── static/                 # CSS 样式
```

## 使用流程

1. 启动服务后访问 `http://127.0.0.1:8080`
2. 在"设置"页面绑定 GitHub 账号（输入用户名和 PAT）
3. 在"项目"页面添加要跟踪的仓库（如 `rust-lang/rust`）
4. 点击"立即同步"或等待自动同步
5. 查看排行榜和贡献详情

## License

MIT
