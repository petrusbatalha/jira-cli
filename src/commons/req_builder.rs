use reqwest::header::CONTENT_TYPE;
use reqwest::{RequestBuilder};
use crate::commons::structs::AuthOptions;
use reqwest::Url;

pub fn build_req(uri: Url, auth_options: &AuthOptions) -> RequestBuilder {
    let client = reqwest::Client::new();
    client.get(uri)
        .basic_auth(auth_options.user.as_ref().unwrap(),auth_options.pass.clone())
        .header(CONTENT_TYPE, "application/json")
}