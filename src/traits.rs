use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ArgOptions {
    pub host: String,
    pub user: Option<String>,
    pub pass: Option<String>,
}

impl Default for ArgOptions {
    fn default() -> Self {
        ArgOptions {
            host: "localhost".to_string(),
            user: None,
            pass: None,
        }
    }
}

#[async_trait]
pub trait Searchable<O, R> {
    type Result = R;
    type Options = O;
    async fn list(&self, options: &O, fixed_options: &ArgOptions, client: &Client) -> R;
}
