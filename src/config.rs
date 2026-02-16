//! Load configuration from environment variables.

use std::net::SocketAddr;

/// Application configuration.
#[derive(Clone, Debug)]
pub struct Config {
    /// Database connection URL.
    pub database_url: String,
    /// HTTP bind address.
    pub bind: SocketAddr,
    /// Log level (RUST_LOG).
    pub log_level: String,
}

impl Config {
    /// Load from environment; returns error if DATABASE_URL is missing.
    pub fn from_env() -> Result<Self, String> {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL is not set".to_string())?;
        let bind = std::env::var("APP_BIND").unwrap_or_else(|_| "127.0.0.1:8080".into());
        let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into());
        let bind: SocketAddr = bind
            .parse()
            .map_err(|e| format!("invalid APP_BIND: {}", e))?;
        Ok(Self {
            database_url,
            bind,
            log_level,
        })
    }

    /// Load with defaults for local development (uses default DB URL if DATABASE_URL is unset).
    pub fn from_env_or_default() -> Self {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "mysql://root:password@127.0.0.1:3306/testr".into());
        let bind = std::env::var("APP_BIND").unwrap_or_else(|_| "127.0.0.1:8080".into());
        let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into());
        let bind: SocketAddr = bind.parse().unwrap_or(([127, 0, 0, 1], 8080).into());
        Self {
            database_url,
            bind,
            log_level,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn from_env_or_default_has_bind_and_db_url() {
        let c = Config::from_env_or_default();
        assert!(!c.database_url.is_empty());
        assert!(c.bind.port() > 0);
    }
}
