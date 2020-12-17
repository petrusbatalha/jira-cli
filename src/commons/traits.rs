use async_trait::async_trait;
use reqwest::Client;
use crate::commons::structs::AuthOptions;
use crate::commons::custom_fields::CustomFieldsCache;

#[async_trait]
pub trait Searchable<O> {
    type Options = O;
    async fn list(
        &self,
        options: &O,
        fixed_options: &AuthOptions,
        custom_fields_cache: &CustomFieldsCache,
    );
}
