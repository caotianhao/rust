//! Route configuration.

use actix_web::web;
use actix_web::Scope;

use crate::handlers::{create_user, delete_user, get_user, health_check, list_users, ready, update_user};

/// Register all API routes.
pub fn configure() -> Scope {
    web::scope("")
        .service(health_check)
        .service(ready)
        .route("/users", web::get().to(list_users))
        .route("/users", web::post().to(create_user))
        .route("/users/{id}", web::get().to(get_user))
        .route("/users/{id}", web::put().to(update_user))
        .route("/users/{id}", web::delete().to(delete_user))
}
