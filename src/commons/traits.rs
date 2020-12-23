use crate::commons::structs::AuthOptions;
use async_trait::async_trait;

#[async_trait]
pub trait Searchable<O> {
    type Options = O;
    async fn list(&self, options: &O, fixed_options: &AuthOptions);
}
