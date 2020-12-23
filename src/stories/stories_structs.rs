use crate::commons::structs::{Fields, Issue, IssueType};
use crate::projects::projects_structs::Project;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::default::default;
use serde_json::Value;

pub struct StoriesHandler;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectKey {
    pub key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stories {
    #[serde(rename = "issueUpdates")]
    pub issue_updates: Vec<StoryRequestFields>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StoryRequestFields {
    pub fields: StoryRequest,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StoryResponse {
    pub expand: Option<String>,
    #[serde(rename = "startAt")]
    pub start_at: Option<i32>,
    #[serde(rename = "maxResults")]
    pub max_result: Option<i32>,
    pub total: Option<i32>,
    pub issues: Option<Vec<Issue>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StoryRequest {
    pub project: Option<ProjectKey>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub issuetype: Option<IssueType>,
    #[serde(rename = "Story Point")]
    pub story_point: Option<i8>,
    pub labels: Option<Vec<String>>,
    #[serde(flatten)]
    pub custom_fields: Option<HashMap<String, Value>>,
}

impl StoryRequestFields {
    pub(crate) fn new_or_template(
        story: StoryRequest,
        story_template: StoryRequest,
    ) -> StoryRequestFields {
        StoryRequestFields {
            fields: StoryRequest {
                project: story.project.or(story_template.project),
                summary: story.summary.or(story_template.summary),
                description: story.description.or(story_template.description),
                story_point: story.story_point.or(story_template.story_point),
                labels: story.labels.or(story_template.labels),
                custom_fields: story.custom_fields.or(story_template.custom_fields),
                ..default()
            },
        }
    }
}

impl Default for StoryRequest {
    fn default() -> Self {
        StoryRequest {
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
