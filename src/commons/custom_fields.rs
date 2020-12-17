use crate::commons::file_utilities::{json_from_file, json_to_file};
use crate::commons::req_builder::build_req;
use crate::commons::structs::{AuthOptions, REST_URI};
use crate::issues::project::Project;
use anyhow::bail;
use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

const FILE_CACHE_PATH: &str = "./.jira-cli/custom_fields";
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
        cache_path: String,
    ) -> Result<CustomFieldsCache, anyhow::Error> {
        let url = Url::parse(&format!(
            "{}{}{}projectKeys={}&expand=projects.issuetypes.fields",
            &auth_options.host, &REST_URI, &SEARCH_URI, project,
        ))
        .unwrap();

        let fields = build_req(url, auth_options)
            .send()
            .await
            .unwrap()
            .json::<ProjectCustomFields>()
            .await
            .unwrap();

        let fields =
            fields.projects[0].clone().issuetypes.unwrap()[0]
            .clone()
            .fields
            .unwrap()
            .unmapped_fields;

        let mut custom_fields_map: HashMap<String, String> = HashMap::new();

        for (key, value) in fields {
            if key.contains("customfield") {
                let new_key = value.get("name").unwrap().as_str().unwrap();
                custom_fields_map.insert(new_key.to_string(), key);
            }
        }

        match json_to_file::<&CustomFieldsCache>(&custom_fields_map, &cache_path).await {
            Ok(()) => {
                debug!("Custom Fields Cache File created at {}", &cache_path);
            }
            Err(e) => {
                bail!("Failed to create Custom Field File Cache {}", e);
            }
        };
        Ok(custom_fields_map)
    }

    pub async fn get_or_cache(
        &self,
        auth_options: &AuthOptions,
        project: &str,
    ) -> Result<CustomFieldsCache, anyhow::Error> {
        let cache_path = format!("{}_{}.json", &FILE_CACHE_PATH, &project);
        match json_from_file::<CustomFieldsCache>(&cache_path).await {
            Ok(fields_result) => match fields_result {
                Ok(fields) => Ok(fields),
                _ => bail!("Failed to create most used fields cache: {}"),
            },
            Err(e) => match self
                .save_custom_fields(auth_options, project, cache_path.clone())
                .await
            {
                Ok(cache) => {
                    info!("Most used fields cache created with success. {}", e);
                    Ok(cache)
                }
                Err(e) => {
                    bail!("Failed to create most used fields cache: {}", e);
                }
            },
        }
    }
}
