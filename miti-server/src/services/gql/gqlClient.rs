use anyhow::Result;
use reqwest::Client;

pub struct GqlClient {
    client: Client,
}

impl GqlClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent("graphql-rust/0.10.0")
            .build()
            .unwrap();
        Ok(Self { client })
    }

    pub async fn request(&self) -> Result<()> {
        Ok(())
    }
}
