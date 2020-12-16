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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fields {
    pub summary: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
