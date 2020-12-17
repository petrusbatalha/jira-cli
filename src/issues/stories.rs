use crate::commons::{
    file_utilities::load_yaml,
    structs::{AuthOptions, Issue, IssueType, Project, JQL, REST_URI},
    traits::Searchable,
};
use crate::commons::custom_fields::CustomFieldsCache;
use crate::{StoryListOps, StoryOps};
use async_trait::async_trait;
use json_patch::merge;
use serde::{Serialize, Deserialize};
use serde_json::{json, Map, Value};
use std::{collections::HashMap, default::default};
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};
use reqwest::Url;
use crate::commons::req_builder::build_req;

pub struct StoriesHandler;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Story {
    pub project: Option<Project>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub issuetype: Option<IssueType>,
    pub labels: Option<Vec<String>>,
    pub custom_fields: Option<Vec<HashMap<String, String>>>,
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
impl Searchable<StoryListOps> for StoriesHandler {
    async fn list(
        &self,
        options: &StoryListOps,
        auth_options: &AuthOptions,
        custom_fields_cache: &CustomFieldsCache,
    ) {
        let uri = format!("{}{}", &auth_options.host, &REST_URI);

        let epic_uri = format!(
            "{}{}{}{}{}",
            &uri,
            &JQL,
            &custom_fields_cache.get("Epic Link").unwrap()[0],
            "=",
            &options.epic.clone()
        );

        let url = Url::parse(&epic_uri).unwrap();

        debug!("Epic Request {}", url);

        let stories = build_req(url, auth_options)
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
        mut args: StoryOps,
        _auth_options: &AuthOptions,
        custom_field_cache: &CustomFieldsCache,
    ) {
        let story_template: Story = match load_yaml(
            &args
                .template_path
                .get_or_insert("./template.yaml".to_string()),
        )
        .await
        {
            Ok(yaml) => {
                serde_yaml::from_str(&yaml).unwrap()
            }
            Err(_) => Story { ..default() },
        };

        let project = match args.project {
            Some(key) => Project::new(key),
            None => story_template.project.unwrap(),
        };

        let mut story_json_fields = json!({
                "project": project,
                "summary": args.summary.or(story_template.summary),
                "description":  args.description.or(story_template.description),
                "labels": args.labels.or(story_template.labels),
        });

        let fields_cache = custom_field_cache.clone();

        let json: Option<Map<String, Value>> = if story_template.custom_fields.is_some() {
            let mut map: Map<String, Value> = Map::new();
            for field_map in story_template.custom_fields.unwrap() {
                for (field_key, field_value) in field_map {
                    let custom_field_key = fields_cache.get(&*field_key).unwrap()[0].clone();
                    let custom_field_name = custom_field_key
                        .clone()
                        .replace("[", "")
                        .replace("]", "")
                        .replace("cf", "customfield_");
                    map.insert(custom_field_name, json!(field_value.clone()));
                }
            }
            Some(map)
        } else {
            None
        };

        let payload: serde_json::Value = match json {
            Some(json) => {
                merge(&mut story_json_fields, &serde_json::to_value(json).unwrap());
                json!({ "fields": story_json_fields })
            }
            None => story_json_fields,
        };
        println!("{:?}", payload.to_string());
    }
}
