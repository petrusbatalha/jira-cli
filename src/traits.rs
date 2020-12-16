use crate::custom_fields::CustomFieldsCache;
use async_trait::async_trait;
use reqwest::Client;
use crate::jira_structs::AuthOptions;

#[async_trait]
pub trait Searchable<O, R> {
    type Result = R;
    type Options = O;
    async fn list(&self, options: &O, fixed_options: &AuthOptions,
                  custom_fields_cache: &CustomFieldsCache, client: &Client) -> R;
}
