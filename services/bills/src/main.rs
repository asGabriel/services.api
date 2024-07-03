use handlers::payments::{payments::payment_service_server::PaymentServiceServer, PaymentServiceImpl};
use tonic::transport::Server;

mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let payment = PaymentServiceImpl::default();

    Server::builder()
        .add_service(PaymentServiceServer::new(payment))
        .serve(addr)
        .await?;

    Ok(())
}
