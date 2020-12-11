use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ArgOptions {
    pub project: Option<String>,
    pub host: String,
    pub user: Option<String>,
    pub pass: Option<String>,
}

impl Default for ArgOptions {
    fn default() -> Self {
        ArgOptions {
            project: None,
            host: "localhost".to_string(),
            user: None,
            pass: None,
        }
    }
}

#[async_trait]
pub trait Searchable<T> {
    type Output = T;
    async fn list(&self, options: &ArgOptions, client: &Client) -> T;
}
