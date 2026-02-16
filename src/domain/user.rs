//! 用户领域模型与 DTO

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 用户实体（与表一致）
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub age: i32,
    pub ctime: i64,
    pub utime: i64,
}

/// 创建用户请求体
#[derive(Debug, Deserialize)]
pub struct UserCreate {
    pub name: String,
    pub age: i32,
}

/// 更新用户请求体
#[derive(Debug, Deserialize)]
pub struct UserUpdate {
    pub name: Option<String>,
    pub age: Option<i32>,
}
