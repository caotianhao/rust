//! HTTP request handlers.

mod health;
mod solana;
mod user;

pub use health::{health as health_check, ready};
pub use solana::{solana_balance, solana_health, solana_slot};
pub use user::{create_user, delete_user, get_user, list_users, update_user};
