//! User CRUD handlers.

use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::MySqlPool;

use crate::domain::{User, UserCreate, UserUpdate};
use crate::error::{AppError, AppResult};

fn validate_user_create(body: &UserCreate) -> AppResult<()> {
    let name = body.name.trim();
    if name.is_empty() {
        return Err(AppError::Validation("name must be non-empty".into()));
    }
    if name.len() > 50 {
        return Err(AppError::Validation("name must be at most 50 characters".into()));
    }
    if body.age <= 0 {
        return Err(AppError::Validation("age must be greater than 0".into()));
    }
    Ok(())
}

fn validate_user_id(id: i64) -> AppResult<()> {
    if id <= 0 {
        return Err(AppError::BadRequest("user id must be a positive integer".into()));
    }
    Ok(())
}

pub async fn list_users(pool: web::Data<MySqlPool>) -> AppResult<impl Responder> {
    let users = sqlx::query_as::<_, User>(
        "SELECT id, name, age, ctime, utime FROM users ORDER BY id",
    )
    .fetch_all(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_user(
    pool: web::Data<MySqlPool>,
    id: web::Path<i64>,
) -> AppResult<impl Responder> {
    validate_user_id(*id)?;
    let user = sqlx::query_as::<_, User>("SELECT id, name, age, ctime, utime FROM users WHERE id = ?")
        .bind(*id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| AppError::NotFound(format!("user id {}", id)))?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn create_user(
    pool: web::Data<MySqlPool>,
    body: web::Json<UserCreate>,
) -> AppResult<impl Responder> {
    validate_user_create(&body)?;
    let now = Utc::now().timestamp();
    let result = sqlx::query(
        "INSERT INTO users (name, age, ctime, utime) VALUES (?, ?, ?, ?)",
    )
    .bind(&body.name)
    .bind(body.age)
    .bind(now)
    .bind(now)
    .execute(pool.get_ref())
    .await?;
    let id = result.last_insert_id() as i64;
    tracing::info!("created user id={} name={}", id, body.name);
    Ok(HttpResponse::Created().json(serde_json::json!({ "id": id })))
}

pub async fn update_user(
    pool: web::Data<MySqlPool>,
    id: web::Path<i64>,
    body: web::Json<UserUpdate>,
) -> AppResult<impl Responder> {
    validate_user_id(*id)?;
    if let Some(ref name) = body.name {
        let t = name.trim();
        if t.is_empty() {
            return Err(AppError::Validation("name must be non-empty".into()));
        }
        if t.len() > 50 {
            return Err(AppError::Validation("name must be at most 50 characters".into()));
        }
    }
    if let Some(age) = body.age {
        if age <= 0 {
            return Err(AppError::Validation("age must be greater than 0".into()));
        }
    }
    let existing = sqlx::query_as::<_, User>("SELECT id, name, age, ctime, utime FROM users WHERE id = ?")
        .bind(*id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| AppError::NotFound(format!("user id {}", id)))?;

    let name = body
        .name
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(existing.name.as_str());
    let age = body.age.unwrap_or(existing.age);
    let now = Utc::now().timestamp();

    sqlx::query("UPDATE users SET name = ?, age = ?, utime = ? WHERE id = ?")
        .bind(name)
        .bind(age)
        .bind(now)
        .bind(*id)
        .execute(pool.get_ref())
        .await?;
    tracing::info!("updated user id={}", id);
    Ok(HttpResponse::Ok().json(serde_json::json!({ "updated": true })))
}

pub async fn delete_user(
    pool: web::Data<MySqlPool>,
    id: web::Path<i64>,
) -> AppResult<impl Responder> {
    validate_user_id(*id)?;
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(*id)
        .execute(pool.get_ref())
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("user id {}", id)));
    }
    tracing::info!("deleted user id={}", id);
    Ok(HttpResponse::Ok().json(serde_json::json!({ "deleted": true })))
}
