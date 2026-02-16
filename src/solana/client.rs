//! Solana RPC client wrapper: timeout, commitment, and error mapping.

use crate::config::SolanaConfig;
use crate::error::{AppError, AppResult};
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use std::sync::Arc;

/// Shared Solana RPC client for use in handlers.
/// RpcClient is synchronous; handlers run it via `actix_web::web::block`.
#[derive(Clone)]
pub struct SolanaClient {
    inner: Arc<RpcClient>,
}

impl SolanaClient {
    /// Build client from config. Returns error if commitment string is invalid.
    pub fn new(config: &SolanaConfig) -> AppResult<Self> {
        let commitment = CommitmentConfig::from_str(&config.commitment)
            .map_err(|e| AppError::BadRequest(format!("invalid SOLANA_COMMITMENT: {}", e)))?;
        let client = RpcClient::new_with_timeout_and_commitment(
            config.rpc_url.clone(),
            config.rpc_timeout,
            commitment,
        );
        Ok(Self {
            inner: Arc::new(client),
        })
    }

    /// Current slot at configured commitment (used for liveness).
    pub fn get_slot(&self) -> Result<u64, solana_client::client_error::ClientError> {
        self.inner.get_slot()
    }

    /// Account balance in lamports.
    pub fn get_balance(&self, pubkey: &Pubkey) -> Result<u64, solana_client::client_error::ClientError> {
        self.inner.get_balance(pubkey)
    }

    /// Latest blockhash (for cluster reachability and tx building).
    pub fn get_latest_blockhash(&self) -> Result<solana_sdk::hash::Hash, solana_client::client_error::ClientError> {
        self.inner.get_latest_blockhash()
    }
}

/// Optional Solana client for app_data: None when SOLANA_RPC_URL is not set.
#[derive(Clone)]
pub enum SolanaClientState {
    Disabled,
    Enabled(SolanaClient),
}

impl SolanaClientState {
    pub fn from_config(config: &Option<SolanaConfig>) -> AppResult<Self> {
        match config {
            None => Ok(SolanaClientState::Disabled),
            Some(cfg) => {
                let client = SolanaClient::new(cfg)?;
                Ok(SolanaClientState::Enabled(client))
            }
        }
    }

    /// Returns the client if Solana is enabled; use this to avoid conflict with `AsRef::as_ref`.
    pub fn client(&self) -> Option<&SolanaClient> {
        match self {
            SolanaClientState::Disabled => None,
            SolanaClientState::Enabled(c) => Some(c),
        }
    }
}
