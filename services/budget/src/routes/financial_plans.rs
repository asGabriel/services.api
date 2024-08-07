use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};

use crate::{
    domains::{errors::Result, financial_plans::CreateFinancialPlan},
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/financial_plans",
        Router::new().route("/", post(create_financial_plan)),
    )
}

async fn create_financial_plan(
    State(handler): State<Handler>,
    Json(payload): Json<CreateFinancialPlan>,
) -> Result<impl IntoResponse> {
    let financial_plan = handler.create_financial_plan(payload).await?;

    Ok(Json(financial_plan))
}
