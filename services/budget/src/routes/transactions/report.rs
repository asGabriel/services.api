use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Deserialize;

use crate::{
    domains::{errors::Result, transactions::report::PeriodFilter},
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest("/report", Router::new().route("/", get(report_data)))
}

#[derive(Debug, Deserialize)]
struct ReportQuery {
    month: i16,
    year: i16,
}

async fn report_data(
    State(handler): State<Handler>,
    Query(query): Query<ReportQuery>,
) -> Result<impl IntoResponse> {
    let transactions = handler
        .generate_report(PeriodFilter {
            month: query.month,
            year: query.year.into(),
        })
        .await?;

    Ok(Json(transactions))
}
