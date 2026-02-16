//! Route configuration.

use actix_web::web;
use actix_web::web::ServiceConfig;

use crate::handlers::{
    create_user, delete_user, get_user, health_check, list_users, ready, solana_balance,
    solana_health, solana_slot, update_user,
};

/// Register all API routes (called by `.configure()` with `&mut ServiceConfig`).
pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(health_check)
            .service(ready)
            .route("/users", web::get().to(list_users))
            .route("/users", web::post().to(create_user))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users/{id}", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
            .route("/solana/health", web::get().to(solana_health))
            .route("/solana/slot", web::get().to(solana_slot))
            .route("/solana/balance/{address}", web::get().to(solana_balance)),
    );
}
