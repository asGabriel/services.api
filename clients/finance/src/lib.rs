use reqwest::Client;

pub mod finance;

#[derive(Debug, Clone, Default)]
pub struct FinanceClient {
    client: Client,
    url: String,
}

impl FinanceClient {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let base_url = std::env::var("BASE_URL").expect("Missing BASE_URL env var");

        FinanceClient {
            client: Client::new(),
            url: base_url,
        }
    }
}
