//! Application-wide error type and HTTP response conversion.

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

/// API error payload for JSON responses.
#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// Application-level error for handlers and services.
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

    #[error("validation error: {0}")]
    Validation(String),

    #[error("Solana RPC error: {0}")]
    Solana(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, body) = match self {
            AppError::Db(e) => {
                tracing::error!("database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorBody {
                        error: "database error".into(),
                        detail: None,
                    },
                )
            }
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                ErrorBody {
                    error: "not found".into(),
                    detail: Some(msg.clone()),
                },
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorBody {
                    error: "bad request".into(),
                    detail: Some(msg.clone()),
                },
            ),
            AppError::Internal(e) => {
                tracing::error!("internal error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorBody {
                        error: "internal error".into(),
                        detail: None,
                    },
                )
            }
            AppError::Validation(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorBody {
                    error: "validation error".into(),
                    detail: Some(msg.clone()),
                },
            ),
            AppError::Solana(msg) => {
                tracing::error!("Solana RPC error: {}", msg);
                (
                    StatusCode::BAD_GATEWAY,
                    ErrorBody {
                        error: "Solana RPC error".into(),
                        detail: Some(msg.clone()),
                    },
                )
            }
        };
        HttpResponse::build(status).json(body)
    }
}
