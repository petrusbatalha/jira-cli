use crate::file_utilities::{json_from_file, json_to_file};
use crate::jira_structs::REST_URI;
use crate::traits::ArgOptions;
use anyhow::bail;
use anyhow::Error;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::default::default;

const MOST_USED_FIELDS: [&'static str; 2] = ["Epic Link", "Team"];
const MOST_USED_FIELDS_PATH: &str = "./most_used_fields.json";
const FILE_CACHE_PATH: &str = "./custom_fields.json";
const FIELD_URI: &str = "/field";

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
        options: &ArgOptions,
        client: &Client,
    ) -> Result<(), anyhow::Error> {
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
        let mut most_used_fields: CustomFieldsCache = HashMap::new();

        for field in fields.clone() {
            if MOST_USED_FIELDS.contains(&&*field.name.clone()) {
                most_used_fields.insert(field.name, field.clause_names);
            } else {
                &custom_fields_map.insert(field.name, field.clause_names);
            }
        }

        match json_to_file::<&CustomFieldsCache>(&most_used_fields, MOST_USED_FIELDS_PATH).await {
            Ok(()) => {
                debug!(
                    " Most Used Fields File created at {}",
                    &MOST_USED_FIELDS_PATH
                );
            }
            Err(e) => {
                error!("Failed to create Most Used Fields File Cache {}", e);
            }
        }

        match json_to_file::<&CustomFieldsCache>(&custom_fields_map, &FILE_CACHE_PATH).await {
            Ok(()) => {
                debug!("Custom Fields Cache File created at {}", &FILE_CACHE_PATH);
            }
            Err(e) => {
                error!("Failed to create Custom Field File Cache {}", e);
            }
        }
        Ok(())
    }

    pub async fn get_custom_field(&self, field: &str) -> Result<Vec<String>, anyhow::Error> {
        if MOST_USED_FIELDS.contains(&field) {
            match json_from_file::<CustomFieldsCache>(&MOST_USED_FIELDS_PATH).await {
                Ok(file) => {
                    Ok(file.unwrap().get(field.clone()).unwrap().clone())
                }
                _ => bail!("Field not found".to_string()),
            }
        } else {
            match json_from_file::<CustomFieldsCache>(&FILE_CACHE_PATH).await {
                Ok(file) => {
                    Ok(file.unwrap().get(field.clone()).unwrap().clone())
                }
                _ => bail!("Field not found".to_string()),
            }
        }
    }

    pub async fn cache_custom_fields(
        &self,
        arg_options: &ArgOptions,
        client: &Client,
    ) -> Result<(), serde_json::Error> {
        match json_from_file::<CustomFieldsCache>(&MOST_USED_FIELDS_PATH).await {
            Ok(_most_used_fields) => {
                match json_from_file::<CustomFieldsCache>(&FILE_CACHE_PATH).await {
                    Ok(_fields) => Ok(()),
                    _ => {
                        self.save_custom_fields(arg_options, client).await.unwrap();
                        Ok(())
                    }
                }
            }
            _ => {
                self.save_custom_fields(arg_options, client).await.unwrap();
                Ok(())
            }
        }
    }
}
