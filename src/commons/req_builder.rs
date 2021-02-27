use crate::commons::structs::AuthOptions;
use reqwest::header::CONTENT_TYPE;
use reqwest::RequestBuilder;
use reqwest::Url;

pub fn build_get_req(uri: Url, auth_options: &AuthOptions) -> RequestBuilder {
    let client = reqwest::Client::new();
    client
        .get(uri)
        .basic_auth(
            auth_options.user.as_ref().unwrap(),
            auth_options.pass.clone(),
        )
        .header(CONTENT_TYPE, "application/json")
}

pub fn build_post_req(uri: Url, auth_options: &AuthOptions) -> RequestBuilder {
    let client = reqwest::Client::new();
    client
        .post(uri)
        .basic_auth(
            auth_options.user.as_ref().unwrap(),
            auth_options.pass.clone(),
        )
        .header(CONTENT_TYPE, "application/json")
}
