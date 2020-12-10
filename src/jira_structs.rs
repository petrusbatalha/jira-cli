use serde::{Deserialize, Serialize};
use serde_json::Value;

pub static REST_URI: &str = "/rest/api/2";
pub static JQL: &str = "/search?jql=";

#[derive(Debug, Clone, Deserialize)]
pub struct Issue {
    pub expand: String,
    pub id: String,
    #[serde(rename = "self")]
    pub issue_link: String,
    pub key: String,
    pub fields: Option<Fields>,
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

#[derive(Debug, Clone, Serialize)]
pub struct FieldsMeta {
    pub issuetype: IssueType,
    pub labels: Vec<String>,
    pub components: Vec<Component>,
    pub project: Project,
    pub customfield_10214: String,
    pub customfield_10101: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fields {
    pub summary: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Project {
    pub key: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct IssueType {
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Component {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IssuesResponse {
    pub issues: Vec<Value>,
}
