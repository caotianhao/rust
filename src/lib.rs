//! testr — production-style Actix + SQLx service.
//!
//! Provides HTTP API, MySQL persistence, unified error handling and config.

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

/// Create MySQL connection pool.
pub async fn create_pool(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}

/// Run table setup (dev/demo; for production prefer sqlx migrate).
pub async fn run_migrations(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'auto-increment id',
            name varchar(50) NOT NULL DEFAULT '' COMMENT 'user name',
            age int(11) NOT NULL DEFAULT 0 COMMENT 'user age',
            ctime bigint(20) NOT NULL DEFAULT 0 COMMENT 'created at',
            utime bigint(20) NOT NULL DEFAULT 0 COMMENT 'updated at',
            PRIMARY KEY (id)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='users table';
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}
