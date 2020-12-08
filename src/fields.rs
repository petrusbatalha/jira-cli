use crate::traits::Searchable;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use async_trait::async_trait;
use crate::jira_structs::{JiraMeta, Schema};
use serde::{Deserialize};
use std::collections::HashMap;
use crate::file_utilities::json_to_file;
use std::collections::hash_map::RandomState;
use serde::de::value::Error;

const FIELDS_URI: &str = "/rest/api/2/field";
const FILE_CACHE_PATH: &str = "./custom_fields.json";
type CustomFieldsCache = HashMap<String, Vec<String>>;

pub struct CustomFieldsHandler {
    pub jira_meta: JiraMeta,
}

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
impl Searchable<Vec<CustomFields>> for CustomFieldsHandler {
    async fn list(&self, client: &Client) -> Result<Vec<CustomFields>, Box<dyn std::error::Error>> {
        let uri = format!("{}{}", &self.jira_meta.host, &FIELDS_URI);

        let fields = client
            .get(&uri)
            .basic_auth(&self.jira_meta.user, Some(&self.jira_meta.pass))
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await?
            .json::<Vec<CustomFields>>()
            .await
            .unwrap();

        let len = *&fields.len();

        let mut custom_fields_map: CustomFieldsCache = HashMap::with_capacity(len);

        for field in fields.clone() {
            custom_fields_map.insert(field.name, field.clause_names);
        }

        match json_to_file::<CustomFieldsCache>(custom_fields_map,&FILE_CACHE_PATH).await {
            Ok(file) => {
                debug!("Custom Fields Cache File created at {}", &FILE_CACHE_PATH);
            },
            Err(e) => {
                error!("Failed to create Custom Field File Cache {}", e);
            }
        }

        Ok(fields)
    }
}
