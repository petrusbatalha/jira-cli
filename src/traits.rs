use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait Searchable {
    async fn list(&self, client: &Client) -> Result<(), Box<dyn std::error::Error>>;
}
