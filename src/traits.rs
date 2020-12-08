use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait Searchable<T> {
    type Output = T;
    async fn list(&self, client: &Client) -> Result<T, Box<dyn std::error::Error>>;
}
