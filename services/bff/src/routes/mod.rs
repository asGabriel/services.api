use std::sync::Arc;
pub mod budget;

use axum::{http::StatusCode, response::IntoResponse, Router};

use crate::{domains::errors::Error, handlers::budget::DynBudgetHandler};

pub(crate) fn configure_services() -> Router<AppState> {
    Router::new().nest("/bff", Router::new().merge(budget::configure_router()))
}

#[derive(Clone)]
pub struct AppState {
    pub budget_handler: Arc<DynBudgetHandler>,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::ReqwestError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
        }
        .into_response()
    }
}
