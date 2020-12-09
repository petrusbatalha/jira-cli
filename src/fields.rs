use crate::file_utilities::{json_from_file, json_to_file};
use crate::jira_structs::{JiraMeta, Schema};
use crate::traits::Searchable;
use async_trait::async_trait;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::de::value::Error;
use serde::Deserialize;
use std::collections::hash_map::{Drain, RandomState};
use std::collections::HashMap;

const FIELDS_URI: &str = "/rest/api/2/field";
const FILE_CACHE_PATH: &str = "./custom_fields.json";

type CustomFieldsCache = HashMap<String, Vec<String>>;

pub struct CustomFieldsHandler {
    pub jira_meta: JiraMeta,
    pub custom_fields: Option<CustomFieldsCache>,
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

impl CustomFieldsHandler {
    async fn get_custom_fields(&self, client: &Client) -> CustomFieldsCache {
        let uri = format!("{}{}", &self.jira_meta.host, &FIELDS_URI);

        let fields = client
            .get(&uri)
            .basic_auth(&self.jira_meta.user, Some(&self.jira_meta.pass))
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .unwrap()
            .json::<Vec<CustomFields>>()
            .await
            .unwrap();

        let len = *&fields.len();

        let mut custom_fields_map: CustomFieldsCache = HashMap::with_capacity(len);

        for field in fields.clone() {
            &custom_fields_map.insert(field.name, field.clause_names);
        }

        match json_to_file::<&CustomFieldsCache>(&custom_fields_map, &FILE_CACHE_PATH).await {
            Ok(()) => {
                debug!("Custom Fields Cache File created at {}", &FILE_CACHE_PATH);
            }
            Err(e) => {
                error!("Failed to create Custom Field File Cache {}", e);
            }
        }
        return custom_fields_map.to_owned();
    }
}

#[async_trait]
impl Searchable<CustomFieldsCache> for CustomFieldsHandler {
    async fn list(&self, client: &Client) -> CustomFieldsCache {
        match json_from_file(&FILE_CACHE_PATH).await {
            Ok(fields) => fields,
            _ => self.get_custom_fields(client).await,
        }
    }
}
