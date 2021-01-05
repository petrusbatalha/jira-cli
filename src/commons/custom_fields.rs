use crate::commons::file_utilities::{json_from_file, json_to_file};
use crate::commons::req_builder::build_get_req;
use crate::commons::structs::{AuthOptions, REST_URI};
use crate::projects::projects_structs::Project;
use anyhow::{anyhow, bail};
use serde::Deserialize;
use serde_json::Error;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use tokio::macros::support::Future;
use url::Url;

const FILE_CACHE_PATH: &str = ".";
const SEARCH_URI: &str = "/issue/createmeta?";

pub type CustomFieldsCache = HashMap<String, String>;

pub struct CustomFieldsHandler;

#[derive(Debug, Clone, Deserialize)]
struct ProjectCustomFields {
    expand: String,
    projects: Vec<Project>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CustomFields {
    id: String,
    name: String,
    custom: Option<bool>,
    orderable: Option<bool>,
    navigable: Option<bool>,
    searchable: Option<bool>,
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
        auth_options: &AuthOptions,
        project: &str,
        cache_path: &str,
        reversed_cache_path: &str,
    ) -> Option<(HashMap<String, String, RandomState>, HashMap<String, String, RandomState>,)> {
        let url = Url::parse(&format!(
            "{}{}{}projectKeys={}&expand=projects.issuetypes.fields",
            &auth_options.host, &REST_URI, &SEARCH_URI, project,
        ))
        .unwrap();

        let fields = build_get_req(url, auth_options)
            .send()
            .await
            .unwrap()
            .json::<ProjectCustomFields>()
            .await
            .unwrap();

        let fields = fields.projects[0].clone().issuetypes.unwrap()[0]
            .clone()
            .fields
            .unwrap()
            .unmapped_fields;

        let mut custom_fields_map: HashMap<String, String> = HashMap::new();
        let mut reversed_fields_map: HashMap<String, String> = HashMap::new();

        for (key, value) in fields {
            if key.contains("customfield") {
                let value = value.get("name").unwrap().as_str().unwrap().to_string();
                custom_fields_map.insert(key.clone(), value.clone());

                let reversed_key = value;
                let parse_key: Vec<&str> = key.split("_").collect();
                let reversed_value = format!("cf[{}]", parse_key[1]);
                reversed_fields_map.insert(reversed_key, reversed_value);
            }
        }

        match json_to_file::<&CustomFieldsCache>(&custom_fields_map, &cache_path).await {
            Ok(r) => {debug!("Custom Fields Cache File created at {}", &cache_path); }
            Err(e) => {error!("Failed to create Custom Field File Cache {}", e); }
        };

        match json_to_file::<&CustomFieldsCache>(&reversed_fields_map, &reversed_cache_path).await {
            Ok(()) => { debug!("Reversed Custom Fields Cache File created at {}", &reversed_cache_path) },
            Err(e) => { error!("Failed to create Reversed Custom Field File Cache {}", e) }
        };
        Some((custom_fields_map, reversed_fields_map))
    }

    pub async fn get_or_cache(
        &self,
        auth_options: &AuthOptions,
        project: &str,
    ) -> Option<(CustomFieldsCache, CustomFieldsCache)> {

        let reversed_cache_path = format!(
            "{}/custom_fields_{}.reversed.json",
            &FILE_CACHE_PATH, &project
        );
        info!("Reversed cache path {}", &reversed_cache_path);

        let cache_path = format!("{}/custom_fields_{}.json", &FILE_CACHE_PATH, &project);
        info!("Cache path {}", &cache_path);

        let custom_fields: Option<CustomFieldsCache> =
            match json_from_file::<CustomFieldsCache>(&cache_path).await {
                Ok(custom_fields) => match custom_fields {
                    Ok(cf) => Some(cf),
                    _ => None,
                },
                _ => None,
            };

        let reversed_custom_fields: Option<CustomFieldsCache> =
            match json_from_file::<CustomFieldsCache>(&reversed_cache_path).await {
                Ok(custom_fields) => match custom_fields {
                    Ok(rcf) => Some(rcf),
                    _ => None,
                },
                _ => None,
            };

        match custom_fields {
            Some(cf) => match reversed_custom_fields {
                Some(rcf) => Some((cf, rcf)),
                _ => None,
            },
            None => {
                match self.save_custom_fields(auth_options, project, &cache_path, &reversed_cache_path, ).await
                { Some(cache) => Some(cache),
                   _ => None,
                }
            }
        }
    }
}