use std::sync::Arc;

use finance_client::finance::FinanceGateway;

pub mod finance;

#[derive(Clone)]
pub struct Handler {
    finance_gateway: Arc<dyn FinanceGateway + Send + Sync>,
}

impl Handler {
    pub const fn new(finance_gateway: Arc<dyn FinanceGateway + Send + Sync>) -> Self {
        Self { finance_gateway }
    }
}
