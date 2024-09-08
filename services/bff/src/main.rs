use std::sync::Arc;

use finance_client::invoices::{FinanceClient, InvoicesGateway};
use handlers::Handler;
pub mod domains;
pub mod gateways;
pub mod handlers;
pub mod routes;

#[tokio::main]
async fn main() {
    let client = Arc::new(FinanceClient::new()) as Arc<dyn InvoicesGateway + Send + Sync>;

    // let app_state = AppState {
    //     budget_handler: Arc::new(budget),
    // };

    let handler = Handler::new(client);

    let router = routes::configure_services().with_state(handler);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    
    axum::serve(listener, router).await.unwrap();
}
