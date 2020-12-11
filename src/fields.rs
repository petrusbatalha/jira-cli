use crate::file_utilities::{json_from_file, json_to_file};
use crate::jira_structs::{Schema, REST_URI};
use crate::traits::{ArgOptions, Searchable};
use async_trait::async_trait;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

const FILE_CACHE_PATH: &str = "./custom_fields.json";
const FIELD_URI: &str = "/fields";

type CustomFieldsCache = HashMap<String, Vec<String>>;

pub struct CustomFieldsHandler;

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
    async fn get_custom_fields(&self, options: &ArgOptions, client: &Client) -> CustomFieldsCache {
        let uri = format!("{}{}{}", &options.host, &REST_URI, &FIELD_URI);

        let fields = client
            .get(&uri)
            .basic_auth(&options.user.as_ref().unwrap(), options.clone().pass)
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
    async fn list(&self, options: &ArgOptions, client: &Client) -> CustomFieldsCache {
        match json_from_file(&FILE_CACHE_PATH).await {
            Ok(fields) => fields,
            _ => self.get_custom_fields(&options, client).await,
        }
    }
}
