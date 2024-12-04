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

        let base_url = std::env::var("BASE_URL")
            .ok()
            .and_then(|v| Some(v))
            .unwrap_or(String::from("http://localhost:8000"));

        FinanceClient {
            client: Client::new(),
            url: base_url,
        }
    }
}
