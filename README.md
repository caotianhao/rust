# testr

生产风格的 Actix Web + SQLx (MySQL) 服务：统一错误处理、配置、健康检查与优雅关闭。

## 结构

- **主二进制**：`cargo run` 启动 HTTP API 服务（默认 `127.0.0.1:8080`）。
- **库**：`src/lib.rs` 暴露 `Config`、`create_pool`、`run_migrations`、`configure`（路由）等，供主程序与测试使用。
- **模块**：`config`（环境配置）、`error`（`AppError` / `AppResult`）、`domain`（User 等）、`handlers`（HTTP 处理）、`routes`（路由注册）。
- **练习/示例**：`src/bin/*.rs` 为独立小示例，`examples/` 为示例程序，不影响主服务。

## 环境与运行

1. 复制环境变量示例并填写数据库连接：
   ```bash
   cp .env.example .env
   # 编辑 .env，设置 DATABASE_URL
   ```
2. 启动服务：
   ```bash
   cargo run
   ```
3. API 前缀为 `/api`：
   - `GET /api/health`、`GET /api/ready`：健康与就绪
   - `GET/POST /api/users`：列表、创建
   - `GET/PUT/DELETE /api/users/{id}`：单用户查询、更新、删除

## 配置

| 变量 | 必填 | 说明 |
|------|------|------|
| `DATABASE_URL` | 是 | MySQL 连接串 |
| `APP_BIND` | 否 | 监听地址，默认 `127.0.0.1:8080` |
| `RUST_LOG` | 否 | 日志级别，默认 `info` |

## 测试

```bash
cargo test
```

运行前需保证 MySQL 可用且 `DATABASE_URL` 已设置（或使用默认库）。

## 依赖概览

- **actix-web**：HTTP 服务
- **sqlx**：MySQL 连接池与查询
- **tokio**：异步运行时
- **tracing / tracing-subscriber**：日志
- **thiserror / anyhow**：错误处理
- **serde / serde_json**：序列化
- **chrono**：时间
- **dotenvy**：`.env` 加载

## License

MIT
