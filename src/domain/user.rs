//! User domain model and DTOs.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// User entity (matches DB table).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub age: i32,
    pub ctime: i64,
    pub utime: i64,
}

/// Request body for creating a user.
#[derive(Debug, Deserialize)]
pub struct UserCreate {
    pub name: String,
    pub age: i32,
}

/// Request body for updating a user.
#[derive(Debug, Deserialize)]
pub struct UserUpdate {
    pub name: Option<String>,
    pub age: Option<i32>,
}
