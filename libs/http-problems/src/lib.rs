use axum::response::IntoResponse;
use errors::Error;
use reqwest::StatusCode;

pub mod errors;

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::ReqwestError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
            Self::NotFoundError(message) => (StatusCode::NOT_FOUND, format!("{message:?}")),
        }
        .into_response()
    }
}
