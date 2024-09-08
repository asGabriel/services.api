use reqwest::Client;

pub struct FinanceClient {
    client: Client,
    url: String,
}

impl FinanceClient {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let base_url = std::env::var("BASE_URL").expect("missing BASE_URL");

        FinanceClient {
            client: Client::new(),
            url: base_url
        }
    }
}

#[async_trait::async_trait]
pub trait InvoicesGateway {
    async fn get_entries(&self) {
        
    }
}
