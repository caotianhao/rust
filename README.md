# testr

生产风格的 Actix Web + SQLx (MySQL) + Solana 服务：统一错误处理、配置、健康检查与优雅关闭。

## 结构

- **主二进制**：`cargo run` 启动 HTTP API 服务（默认 `127.0.0.1:8080`）。
- **库**：`src/lib.rs` 暴露 `Config`、`create_pool`、`run_migrations`、`configure`、`SolanaClientState` 等。
- **模块**：`config`、`error`、`domain`、`handlers`、`routes`、**`solana`**（Solana RPC 封装）。
- **练习/示例**：`src/bin/*.rs` 为独立小示例，`examples/` 为示例程序，不影响主服务。

## 环境与运行

1. 复制环境变量示例并填写：
   ```bash
   cp .env.example .env
   # 编辑 .env：DATABASE_URL 必填；Solana 可选（SOLANA_RPC_URL）
   ```
2. 启动服务：
   ```bash
   cargo run
   ```
3. API 前缀为 `/api`：
   - `GET /api/health`、`GET /api/ready`：健康与就绪
   - `GET/POST /api/users`：列表、创建
   - `GET/PUT/DELETE /api/users/{id}`：单用户查询、更新、删除
   - **Solana**（需配置 `SOLANA_RPC_URL`）：
     - `GET /api/solana/health`：集群存活（slot + blockhash）
     - `GET /api/solana/slot`：当前 slot
     - `GET /api/solana/balance/{address}`：账户余额（lamports + SOL）

## 配置

| 变量 | 必填 | 说明 |
|------|------|------|
| `DATABASE_URL` | 是 | MySQL 连接串 |
| `APP_BIND` | 否 | 监听地址，默认 `127.0.0.1:8080` |
| `RUST_LOG` | 否 | 日志级别，默认 `info` |
| `SOLANA_RPC_URL` | 否 | Solana RPC 地址；不设则禁用 Solana 相关接口 |
| `SOLANA_RPC_TIMEOUT_SECS` | 否 | RPC 超时秒数，默认 30 |
| `SOLANA_COMMITMENT` | 否 | 提交级别：processed / confirmed / finalized，默认 confirmed |
| `SOLANA_KEYPAIR_PATH` | 否 | 可选：手续费支付密钥路径（仅读接口可不设） |

## 测试

```bash
cargo test
```

运行前需保证 MySQL 可用且 `DATABASE_URL` 已设置（或使用默认库）。

## 依赖概览

- **actix-web**：HTTP 服务
- **sqlx**：MySQL 连接池与查询
- **solana-client / solana-sdk**：Solana RPC 与类型（Pubkey、CommitmentConfig 等）
- **tokio**：异步运行时
- **tracing / tracing-subscriber**：日志
- **thiserror / anyhow**：错误处理
- **serde / serde_json**：序列化
- **chrono**：时间
- **dotenvy**：`.env` 加载

## License

MIT
