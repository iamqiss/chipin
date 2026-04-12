use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Payment error: {0}")]
    PaymentError(String),

    #[error("KYC required")]
    KycRequired,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Multi-sig approval required")]
    MultiSigRequired,

    #[error("Fraud detected")]
    FraudDetected,

    #[error("Internal error")]
    Internal(#[from] anyhow::Error),

    #[error("Database error")]
    Database(#[from] sea_orm::DbErr),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(m)       => (StatusCode::NOT_FOUND, m.clone()),
            AppError::Unauthorized      => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
            AppError::Forbidden         => (StatusCode::FORBIDDEN, "Forbidden".into()),
            AppError::BadRequest(m)     => (StatusCode::BAD_REQUEST, m.clone()),
            AppError::Conflict(m)       => (StatusCode::CONFLICT, m.clone()),
            AppError::PaymentError(m)   => (StatusCode::BAD_GATEWAY, m.clone()),
            AppError::KycRequired       => (StatusCode::FORBIDDEN, "KYC verification required".into()),
            AppError::InsufficientFunds => (StatusCode::UNPROCESSABLE_ENTITY, "Insufficient funds".into()),
            AppError::MultiSigRequired  => (StatusCode::FORBIDDEN, "Multi-sig approval required".into()),
            AppError::FraudDetected     => (StatusCode::FORBIDDEN, "Transaction blocked by Fraud Shield".into()),
            AppError::Internal(_)       => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into()),
            AppError::Database(_)       => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
