use crate::commons::structs::{Issue, IssueType};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};
use crate::projects::projects_structs::Project;

pub struct StoriesHandler;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stories {
    pub stories: Vec<Story>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Story {
    pub project: Option<Project>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub issuetype: Option<IssueType>,
    #[serde(rename = "Story Point")]
    pub story_point: Option<i8>,
    pub labels: Option<Vec<String>>,
    pub custom_fields: Option<Vec<BTreeMap<String, String>>>,
}

impl Default for Story {
    fn default() -> Self {
        Story {
            project: None,
            summary: None,
            story_point: Some(0 as i8),
            description: None,
            issuetype: Some(IssueType {
                name: "Story".to_string(),
                fields: None,
            }),
            labels: None,
            custom_fields: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomField {
    name: Option<String>,
    value: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StoryMeta {
    pub expand: Option<String>,
    #[serde(rename = "startAt")]
    pub start_at: Option<i32>,
    #[serde(rename = "maxResults")]
    pub max_result: Option<i32>,
    pub total: Option<i32>,
    pub issues: Option<Vec<Issue>>,
}
