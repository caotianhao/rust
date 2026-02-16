//! Solana RPC handlers: cluster health, balance, slot (read-only, production-safe).

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use crate::error::{AppError, AppResult};
use crate::solana::SolanaClientState;

fn map_solana_err(e: solana_client::client_error::ClientError) -> AppError {
    let msg = e.to_string();
    AppError::Solana(msg)
}

/// GET /solana/health — cluster liveness (slot + blockhash).
#[derive(Serialize)]
struct SolanaHealthResponse {
    ok: bool,
    slot: u64,
    blockhash: String,
}

pub async fn solana_health(state: web::Data<SolanaClientState>) -> AppResult<impl Responder> {
    let client = state
        .client()
        .ok_or_else(|| AppError::BadRequest("Solana RPC not configured (set SOLANA_RPC_URL)".into()))?
        .clone();
    let (slot, blockhash) = actix_web::web::block(move || {
        let slot = client.get_slot().map_err(map_solana_err)?;
        let hash = client.get_latest_blockhash().map_err(map_solana_err)?;
        Ok::<_, AppError>((slot, hash.to_string()))
    })
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("blocking task: {}", e)))??;
    Ok(HttpResponse::Ok().json(SolanaHealthResponse {
        ok: true,
        slot,
        blockhash,
    }))
}

/// GET /solana/slot — current slot only.
#[derive(Serialize)]
struct SolanaSlotResponse {
    slot: u64,
}

pub async fn solana_slot(state: web::Data<SolanaClientState>) -> AppResult<impl Responder> {
    let client = state
        .client()
        .ok_or_else(|| AppError::BadRequest("Solana RPC not configured (set SOLANA_RPC_URL)".into()))?;
    let client = client.clone();
    let slot = actix_web::web::block(move || client.get_slot().map_err(map_solana_err))
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("blocking task: {}", e)))??;
    Ok(HttpResponse::Ok().json(SolanaSlotResponse { slot }))
}

/// GET /solana/balance/:address — account balance in lamports and SOL.
#[derive(Serialize)]
struct SolanaBalanceResponse {
    address: String,
    lamports: u64,
    sol: String,
}

pub async fn solana_balance(
    state: web::Data<SolanaClientState>,
    path: web::Path<String>,
) -> AppResult<impl Responder> {
    let client = state
        .client()
        .ok_or_else(|| AppError::BadRequest("Solana RPC not configured (set SOLANA_RPC_URL)".into()))?;
    let address = path.into_inner();
    let pubkey = Pubkey::from_str(&address)
        .map_err(|_| AppError::BadRequest("invalid Solana address (base58)".into()))?;
    let client = client.clone();
    let lamports = actix_web::web::block(move || client.get_balance(&pubkey).map_err(map_solana_err))
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("blocking task: {}", e)))??;
    let sol = lamports as f64 / 1_000_000_000.0;
    Ok(HttpResponse::Ok().json(SolanaBalanceResponse {
        address,
        lamports,
        sol: format!("{:.9}", sol),
    }))
}
