use axum::{http::StatusCode, response::IntoResponse};
use reqwest::Error as ReqwestError;
use sqlx::Error as SqlxError;
use thiserror::Error;

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::ReqwestError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
            Self::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
            Self::NotFoundError(message) => (StatusCode::NOT_FOUND, format!("{message:?}")),
            Self::BadRequestError(message) => (StatusCode::BAD_REQUEST, format!("{message:?}")),
            Self::UnauthorizedError(message) => (StatusCode::UNAUTHORIZED, format!("{message:?}")),
        }
        .into_response()
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error")]
    DatabaseError(#[from] SqlxError),
    #[error("Reqwest error")]
    ReqwestError(#[from] ReqwestError),
    #[error("NotFoundError")]
    NotFoundError(String),
    #[error("BadRequestError")]
    BadRequestError(String),
    #[error("UnauthorizedError")]
    UnauthorizedError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
