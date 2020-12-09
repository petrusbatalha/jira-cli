use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) static REST_URI: &str = "/rest/api/2";

#[derive(Debug, Clone, Deserialize)]
pub struct Schema {
    #[serde(rename = "type")]
    issue_type: Option<String>,
    custom: Option<String>,
    custom_id: Option<i32>,
    items: Option<String>,
    system: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JiraMeta {
    pub host: String,
    pub user: String,
    pub pass: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct FieldsMeta {
    pub issuetype: IssueType,
    pub labels: Vec<String>,
    pub components: Vec<Component>,
    pub project: Project,
    pub(crate) customfield_10214: String,
    pub(crate) customfield_10101: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Fields {
    pub summary: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Project {
    pub(crate) key: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct IssueType {
    pub(crate) name: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Component {
    pub(crate) name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct IssuesResponse {
    pub(crate) issues: Vec<Value>,
}
