use std::sync::Arc;

use gateways::budget::ApiBudgetGateway;
use handlers::budget::BudgetHandlerImpl;
use routes::AppState;
pub mod domains;
pub mod gateways;
pub mod handlers;
pub mod routes;

fn create_budget_handler() -> BudgetHandlerImpl {
    let budget_gateway = Arc::new(ApiBudgetGateway::default());

    BudgetHandlerImpl { budget_gateway }
}

#[tokio::main]
async fn main() {
    println!("bff-service");

    let budget = create_budget_handler();

    let app_state = AppState {
        budget_handler: Arc::new(budget),
    };

    let router = routes::configure_services().with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
