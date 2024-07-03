use payments::{payment_service_server::PaymentService, CreatePaymentRequest, Payment, PaymentResponse};
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub mod payments {
    tonic::include_proto!("payment");
}

#[derive(Debug, Default)]
pub struct PaymentServiceImpl {}

#[tonic::async_trait]
impl PaymentService for PaymentServiceImpl {
    async fn create_payment(
        &self,
        request: Request<CreatePaymentRequest>,
    ) -> Result<Response<PaymentResponse>, Status> {
        println!("{:?}", request);
        
        let request = request.into_inner();

        let payment = Payment {
            id: Uuid::new_v4().to_string(),
            title: request.title,
            amount: request.amount,
        };

        let response = PaymentResponse {
            payment: Some(payment),
        };

        Ok(Response::new(response))
    }
}
