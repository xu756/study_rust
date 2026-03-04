# Rust API 项目学习说明（基于本项目重构版）

## 1. 这个项目现在的整体结构

这是一个 **Cargo Workspace 多包项目**，根目录下有 4 个 crate：

- `app`：二进制入口（程序启动点）
- `router`：HTTP 路由和接口处理
- `common`：通用能力（数据库、实体、JWT、统一响应、错误）
- `config`：配置加载（`config.toml` + 环境变量覆盖）

你可以把它理解成 Go 项目里的分层：

- `app` ≈ `cmd/server`
- `router` ≈ `transport/http` 或 `api`
- `common` ≈ `pkg` / `internal/shared`
- `config` ≈ `config` + 初始化逻辑

---

## 2. 每个 crate（包）是什么意思

## `app`（启动层）

- 文件：`app/src/main.rs`
- 作用：启动 Tokio 运行时，调用 `router::init_router()` 启动服务。
- 特点：不写业务逻辑，只负责“程序入口和启动失败处理”。

## `router`（接口层）

- 核心文件：
  - `router/src/lib.rs`：组装 Router、挂载 API 前缀、启动监听
  - `router/src/state.rs`：`AppState`（把数据库连接共享给 handler）
  - `router/src/routes/mod.rs`：路由总入口
  - `router/src/routes/index.rs`：示例接口
  - `router/src/routes/user.rs`：用户 API（列表、详情、创建）
- 作用：处理 HTTP 请求/响应，不直接关心配置解析细节。

## `common`（共享能力层）

- `common/src/db/connect.rs`：连接 SQLite，自动创建 `users` 表
- `common/src/db/user.rs`：用户数据访问函数（create/list/find）
- `common/src/entity/user.rs`：SeaORM 实体模型（映射 `users` 表）
- `common/src/jwt/mod.rs`：JWT 生成与校验
- `common/src/result/mod.rs`：统一 JSON 返回格式
- `common/src/error/*`：统一错误码与错误结构

## `config`（配置层）

- `config/src/cfgs.rs`：配置结构体定义（server/database/jwt 等）
- `config/src/get_config.rs`：
  - `CFG` 全局只读配置（`once_cell::Lazy`）
  - 从 `config.toml` 读取
  - 支持环境变量覆盖（如 `SERVER_ADDRESS` / `DATABASE_LINK` / `JWT_SECRET`）

---

## 3. `main.rs` / `lib.rs` / `mod.rs` 是什么

## `main.rs`

- 二进制程序入口。
- 类似 Go 的 `func main()`。
- 一个 crate 如果有 `main.rs`，说明它是“可执行程序”。

## `lib.rs`

- 库 crate 的根模块入口。
- 负责把模块导出给外部使用，例如：
  - `pub mod db;`
  - `pub mod jwt;`
- 其他 crate 通过依赖这个 crate 后，可以使用 `crate_name::xxx` 调用。

## `mod.rs`

- 目录模块入口（传统写法）。
- 例如 `router/src/routes/mod.rs` 表示 `routes` 模块的根。
- 在这个文件里可以继续声明子模块：
  - `mod index;`
  - `mod user;`

说明：Rust 2018+ 也可以不用 `mod.rs`，改成同名文件 + 同名目录组合。两种都能用，本项目保留了 `mod.rs`，更直观，适合初学者。

---

## 4. 当前 API 示例

默认配置下：

- 服务地址：`127.0.0.1:3000`
- API 前缀：`/api`

接口：

- `POST /api/index/`
- `GET /api/users/`
- `GET /api/users/{id}`
- `POST /api/users/`

创建用户示例：

```bash
curl -X POST http://127.0.0.1:3000/api/users/ \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","email":"alice@example.com"}'
```

---

## 5. 你学习这个项目时建议按这个顺序看

1. 先看 `app/src/main.rs`（程序从哪儿开始）
2. 再看 `router/src/lib.rs`（路由怎么装起来）
3. 再看 `router/src/routes/user.rs`（请求如何进入业务）
4. 再看 `common/src/db/user.rs` + `common/src/entity/user.rs`（数据怎么落 SQLite）
5. 最后看 `config`（配置如何注入到整个系统）

---

## 6. 现在这个结构相比你之前代码的改进点

- 修复了 `config::CFG` 缺失导致的不可编译问题
- SQLite 连接与建表流程可用（启动自动建 `users` 表）
- 路由和状态管理清晰分层（`state + routes`）
- 增加了可运行的用户 API 示例，便于你继续迁移 Go 业务
- `cargo check` / `cargo test` 可通过，便于迭代学习

