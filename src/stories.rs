use crate::fields::CustomFieldsHandler;
use crate::file_utilities::{load_yaml};
use crate::jira_structs::{Issue, IssueType, Project, JQL, REST_URI};
use crate::traits::{ArgOptions, Searchable};
use anyhow::{bail, anyhow};
use async_trait::async_trait;
use core::fmt;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_yaml;
use serde_json::{Map, json};
use std::default::default;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{Table, TableStyle};
use yaml_rust::YamlLoader;
use std::borrow::Borrow;
use json_patch::merge;


static STORIES_URI: &str = "/project";

pub struct StoriesHandler;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Story {
    pub project: Option<Project>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub issuetype: Option<IssueType>,
    pub labels: Option<Vec<String>>,
    pub custom_fields: Option<Vec<String>>,
}


impl Default for Story {
    fn default() -> Self {
        Story {
            project: None,
            summary: None,
            description: None,
            issuetype: Some(IssueType {
                name: "Story".to_string(),
            }),
            labels: None,
            custom_fields: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Stories {
    pub expand: Option<String>,
    #[serde(rename = "startAt")]
    pub start_at: Option<i32>,
    #[serde(rename = "maxResults")]
    pub max_result: Option<i32>,
    pub total: Option<i32>,
    pub issues: Vec<Issue>,
}

#[async_trait]
impl Searchable<Result<(), ()>> for StoriesHandler {
    async fn list(&self, options: &ArgOptions, client: &Client) -> Result<(), ()> {
        let uri = format!("{}{}", &options.host, &REST_URI);

        let epic_uri = format!(
            "{}{}{}{}{}",
            &uri,
            &JQL,
            &CustomFieldsHandler
                .get_custom_field("Epic Link")
                .await
                .unwrap()[0],
            "=",
            &options.epic.as_ref().unwrap()
        );

        debug!("Epic Request {}", epic_uri);

        let stories = client
            .get(&epic_uri)
            .basic_auth(&options.user.as_ref().unwrap(), options.clone().pass)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .unwrap()
            .json::<Stories>()
            .await
            .unwrap();

        let mut table = Table::new();
        table.max_column_width = 80;
        table.style = TableStyle::blank();

        table.add_row(build_table_header_row());

        for issue in stories.issues {
            table.add_row(build_table_body(issue));
        }

        println!("{}", table.render());

        Ok(())
    }
}

fn build_table_body(stories: Issue) -> Row<'static> {
    Row::new(vec![
        TableCell::new_with_alignment(stories.fields.unwrap().summary.unwrap(), 1, Alignment::Left),
        TableCell::new_with_alignment(stories.key, 1, Alignment::Left),
        TableCell::new_with_alignment(stories.id, 2, Alignment::Left),
        TableCell::new_with_alignment(stories.issue_link, 1, Alignment::Left),
    ])
}

fn build_table_header_row() -> Row<'static> {
    Row::new(vec![
        TableCell::new_with_alignment("Name", 1, Alignment::Left),
        TableCell::new_with_alignment("Key", 2, Alignment::Left),
        TableCell::new_with_alignment("ID", 1, Alignment::Left),
        TableCell::new_with_alignment("Link", 1, Alignment::Left),
    ])
}

impl StoriesHandler {
    pub async fn create_story(
        &self,
        project: Option<Project>,
        summary: Option<String>,
        description: Option<String>,
        labels: Option<Vec<String>>,
        custom_fields: Option<Vec<String>>,
        mut path: Option<String>,
    ) -> serde_json::Value {
        let mut story_template: Story =
            match load_yaml(&path.get_or_insert("./template.yaml".to_string())).await {
                Ok(yaml) => {
                    let story_yaml = serde_yaml::from_str(&yaml).unwrap();
                    story_yaml
                }
                Err(_) => Story { ..default() },
            };


        let story = Story {
            project: project.or(story_template.project),
            summary: summary.or(story_template.summary),
            description: description.or(story_template.description),
            labels: labels.or(story_template.labels),
            ..default()
        };

        let mut story_json_fields =
            json!(&story);
        let cf =
            custom_fields.or(story_template.custom_fields);

        let json: Option<Map<String, Value>> = if cf.is_some() {
            info!("É SOME");
           let custom_fields = cf.unwrap();
            let mut map: Map<String, Value> = Map::new();
                for field in custom_fields {
                    let custom_fields =
                        &CustomFieldsHandler.get_custom_field(&*field).await.unwrap();
                    let custom_field_name = custom_fields[0].clone()
                        .replace("[", "")
                        .replace("]", "")
                        .replace("cf", "customfield_");
                    &map.insert(custom_field_name, json!(custom_fields[1].clone()));
                };
            Some(map)
        } else {
            None
        };
        let payload: serde_json::Value = match json {
            Some(json) => {
                merge(&mut story_json_fields, &serde_json::to_value(json).unwrap());
                story_json_fields
            },
            None => story_json_fields
        };
        payload
    }
}
