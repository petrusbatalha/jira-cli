use crate::file_utilities::{json_from_file, json_to_file};
use crate::jira_structs::REST_URI;
use crate::jira_structs::AuthOptions;
use anyhow::bail;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use serde_json::Error;

const FILE_CACHE_PATH: &str = "./.jira-cli/custom_fields.json";
const FIELD_URI: &str = "/field";

pub type CustomFieldsCache = HashMap<String, Vec<String>>;

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

#[derive(Debug, Clone, Deserialize)]
pub struct Schema {
    #[serde(rename = "type")]
    issue_type: Option<String>,
    custom: Option<String>,
    custom_id: Option<i32>,
    items: Option<String>,
    system: Option<String>,
}

impl CustomFieldsHandler {
    async fn save_custom_fields(
        &self,
        options: &AuthOptions,
        client: &Client,
    ) -> Result<CustomFieldsCache, anyhow::Error> {
        let uri = format!("{}{}{}", &options.host, &REST_URI, FIELD_URI);

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
                bail!("Failed to create Custom Field File Cache {}", e);
            }
        };
        Ok(custom_fields_map)
    }

    pub async fn cache_custom_fields(
        &self,
        arg_options: &AuthOptions,
        client: &Client,
    ) -> Result<CustomFieldsCache, anyhow::Error> {
        match json_from_file::<CustomFieldsCache>(&FILE_CACHE_PATH).await {
            Ok(fields_result) => match fields_result {
                Ok(fields) => Ok(fields),
                _ => bail!("Failed to create most used fields cache: {}")
            },
            Err(e) => {
                match self.save_custom_fields(arg_options, client).await {
                    Ok(cache) => {
                        info!("Most used fields cache created with success.");
                        Ok(cache)
                    },
                    Err(e) => {
                        bail!("Failed to create most used fields cache: {}", e);
                    }
                }
            }
        }
    }
}
