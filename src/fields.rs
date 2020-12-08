use crate::traits::Searchable;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use async_trait::async_trait;
use crate::jira_structs::{JiraMeta, Schema};
use serde::{Deserialize};


const FIELDS_URI: &str = "/rest/api/2/field";

#[derive(Debug, Clone, Deserialize)]
pub struct CustomFields {
    id: String,
    name: String,
    custom: bool,
    orderable: bool,
    navigable: bool,
    searchable: bool,
    #[serde(rename = "clauseNames")]
    clause_names: Vec<String>,
    schema: Option<Schema>,
}

#[async_trait]
impl Searchable for JiraMeta {
    async fn list(&self, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        let uri = format!("{}{}", &self.host, &FIELDS_URI);

        println!("URI: {:?}", uri);

        let fields = client
            .get(&uri)
            .basic_auth(&self.user, Some(&self.pass))
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await?
            .json::<Vec<CustomFields>>()
            .await;

        println!("Name: {:?}", &fields.unwrap());

        Ok(())
    }
}
