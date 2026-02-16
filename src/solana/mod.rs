//! Production-style Solana RPC integration.
//!
//! Wraps `solana_client::RpcClient` with configurable timeout/commitment,
//! runs blocking RPC calls on a thread pool to avoid blocking the async runtime,
//! and maps all errors to `AppError`.

mod client;

pub use client::SolanaClient;
pub use client::SolanaClientState;
