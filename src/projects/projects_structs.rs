use crate::commons::structs::IssueType;
use serde::{Deserialize, Serialize};
use std::fmt;

pub(crate) static PROJECT_URI: &str = "/project";

pub struct ProjectHandler;

#[derive(Debug, Clone)]
pub struct ProjectDisplay {
    pub name: String,
    pub key: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AvatarUrls {
    #[serde(rename = "16x16")]
    sixteen_url: String,
    #[serde(rename = "24x24")]
    twenty_four_url: String,
    #[serde(rename = "32x32")]
    thirty_two_url: String,
    #[serde(rename = "48x48")]
    forty_eight_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectCategory {
    #[serde(rename = "self")]
    project_category: String,
    id: String,
    name: String,
    description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub expand: Option<String>,
    #[serde(rename = "self")]
    pub project_url: Option<String>,
    pub id: Option<String>,
    pub key: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "avatarUrls")]
    pub avatar_urls: Option<AvatarUrls>,
    #[serde(rename = "projectCategory")]
    pub project_category: Option<ProjectCategory>,
    #[serde(rename = "projectTypeKey")]
    pub project_type_key: Option<String>,
    pub issuetypes: Option<Vec<IssueType>>,
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} \t\t {} \t {} \t", &self.key.as_ref().unwrap(), &self.name.as_ref().unwrap(), &self.id.as_ref().unwrap())
    }
}
