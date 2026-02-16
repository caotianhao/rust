//! Load configuration from environment variables.

use std::net::SocketAddr;
use std::time::Duration;

/// Application configuration.
#[derive(Clone, Debug)]
pub struct Config {
    /// Database connection URL.
    pub database_url: String,
    /// HTTP bind address.
    pub bind: SocketAddr,
    /// Log level (RUST_LOG).
    pub log_level: String,
    /// Solana RPC and client settings (optional).
    pub solana: Option<SolanaConfig>,
}

/// Solana RPC client configuration (production: use env, no hardcoded secrets).
#[derive(Clone, Debug)]
pub struct SolanaConfig {
    /// RPC endpoint URL (e.g. https://api.mainnet-beta.solana.com).
    pub rpc_url: String,
    /// Request timeout for RPC calls.
    pub rpc_timeout: Duration,
    /// Commitment level: "processed", "confirmed", or "finalized".
    pub commitment: String,
    /// Optional fee-payer keypair path (file with base58 or JSON array). Omit for read-only.
    pub keypair_path: Option<String>,
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
        let solana = Self::load_solana_config();
        Ok(Self {
            database_url,
            bind,
            log_level,
            solana,
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
        let solana = Self::load_solana_config();
        Self {
            database_url,
            bind,
            log_level,
            solana,
        }
    }
}

impl Config {
    fn load_solana_config() -> Option<SolanaConfig> {
        let rpc_url = std::env::var("SOLANA_RPC_URL").ok()?;
        if rpc_url.is_empty() {
            return None;
        }
        let timeout_secs = std::env::var("SOLANA_RPC_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(30);
        let commitment = std::env::var("SOLANA_COMMITMENT")
            .unwrap_or_else(|_| "confirmed".into());
        let keypair_path = std::env::var("SOLANA_KEYPAIR_PATH").ok();
        Some(SolanaConfig {
            rpc_url,
            rpc_timeout: Duration::from_secs(timeout_secs),
            commitment,
            keypair_path,
        })
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
