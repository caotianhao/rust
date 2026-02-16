//! testr — 生产风格 Actix + SQLx 服务
//!
//! 提供 HTTP API、MySQL 持久化、统一错误与配置。

pub mod config;
pub mod domain;
pub mod error;
pub mod handlers;
pub mod routes;

pub use config::Config;
pub use error::{AppError, AppResult};
pub use routes::configure;

use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

/// 创建 MySQL 连接池
pub async fn create_pool(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}

/// 执行表结构初始化（开发/演示用；生产建议用 sqlx migrate）
pub async fn run_migrations(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id bigint(20) NOT NULL AUTO_INCREMENT COMMENT '自增ID',
            name varchar(50) NOT NULL DEFAULT '' COMMENT '用户名称',
            age int(11) NOT NULL DEFAULT 0 COMMENT '用户年龄',
            ctime bigint(20) NOT NULL DEFAULT 0 COMMENT '创建时间',
            utime bigint(20) NOT NULL DEFAULT 0 COMMENT '更新时间',
            PRIMARY KEY (id)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户表';
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}
