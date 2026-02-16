//! 应用统一错误类型与 HTTP 响应转换

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;

/// 应用级错误，用于 handler 和 service 层
#[derive(Error, Debug)]
pub enum AppError {
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

pub type AppResult<T> = Result<T, AppError>;

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let err_msg = self.to_string();
        let (status, msg) = match self {
            AppError::Db(e) => {
                tracing::error!("database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "database error")
            }
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, err_msg.as_str()),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, err_msg.as_str()),
            AppError::Internal(e) => {
                tracing::error!("internal error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error")
            }
        };
        HttpResponse::build(status).body(msg.to_string())
    }
}
