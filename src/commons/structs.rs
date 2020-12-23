use crate::stories::stories_structs::Stories;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub static REST_URI: &str = "/rest/api/2";
pub static JQL: &str = "/search?jql=";

#[derive(Debug, Clone, Deserialize)]
pub struct AuthOptions {
    pub host: String,
    pub user: Option<String>,
    pub pass: Option<String>,
}

impl Default for AuthOptions {
    fn default() -> Self {
        AuthOptions {
            host: "localhost".to_string(),
            user: None,
            pass: None,
        }
    }
}

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
pub struct FieldsType {
    #[serde(flatten)]
    pub unmapped_fields: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueType {
    pub name: String,
    pub fields: Option<FieldsType>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Component {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IssuesResponse {
    pub issues: Vec<Value>,
}
