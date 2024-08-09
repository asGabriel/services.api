use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    domains::{errors::Result, financial_plans::CreateFinancialPlan},
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/financial_plans",
        Router::new()
            .route("/", post(create_financial_plan))
            .route("/", get(list_financial_plans))
            .route("/:id", get(get_financial_plan_by_id)),
    )
}

async fn create_financial_plan(
    State(handler): State<Handler>,
    Json(payload): Json<CreateFinancialPlan>,
) -> Result<impl IntoResponse> {
    let financial_plan = handler.create_financial_plan(payload).await?;

    Ok(Json(financial_plan))
}

async fn list_financial_plans(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let financial_plans = handler.list_financial_plans().await?;

    Ok(Json(financial_plans))
}

async fn get_financial_plan_by_id(
    State(handler): State<Handler>,
    Path(financial_plan_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let financial_plan = handler.get_financial_plan_by_id(financial_plan_id).await?;

    Ok(Json(financial_plan))
}
