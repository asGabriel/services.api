use std::sync::Arc;

use finance_client::{finance::FinanceGateway, FinanceClient};
use handlers::Handler;
pub mod domains;
pub mod gateways;
pub mod handlers;
pub mod routes;

#[tokio::main]
async fn main() {
    let finance_gateway = Arc::new(FinanceClient::new()) as Arc<dyn FinanceGateway + Send + Sync>;

    let handler = Handler::new(finance_gateway);

    let router = routes::configure_services().with_state(handler);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
