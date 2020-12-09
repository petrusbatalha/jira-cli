use crate::jira_structs::{JiraMeta, REST_URI};
use crate::traits::Searchable;
use async_trait::async_trait;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::Deserialize;

static PROJECT_URI: &str = "/project";

pub struct ProjectHandler {
    pub jira_meta: JiraMeta,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectDisplay {
    pub name: String,
    pub key: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
struct AvatarUrls {
    #[serde(rename = "16x16")]
    sixteen_url: String,
    #[serde(rename = "24x24")]
    twenty_four_url: String,
    #[serde(rename = "32x32")]
    thirty_two_url: String,
    #[serde(rename = "48x48")]
    forty_eight_url: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ProjectCategory {
    #[serde(rename = "self")]
    project_category: String,
    id: String,
    name: String,
    description: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Project {
    expand: String,
    #[serde(rename = "self")]
    project_url: String,
    id: String,
    key: String,
    name: String,
    #[serde(rename = "avatarUrls")]
    avatar_urls: AvatarUrls,
    #[serde(rename = "projectCategory")]
    project_category: ProjectCategory,
    #[serde(rename = "projectTypeKey")]
    project_type_key: String,
}

#[async_trait]
impl Searchable<Vec<ProjectDisplay>> for ProjectHandler {
    async fn list(&self, client: &Client) -> Vec<ProjectDisplay> {
        let uri = format!("{}{}{}", &self.jira_meta.host, &REST_URI, &PROJECT_URI);

        let projects = client
            .get(&uri)
            .basic_auth(&self.jira_meta.user, Some(&self.jira_meta.pass))
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .unwrap()
            .json::<Vec<Project>>()
            .await
            .unwrap();

        let len = projects.clone().len();
        let mut projects_display: Vec<ProjectDisplay> = Vec::with_capacity(len);

        for project in projects.clone() {
            let project_display = ProjectDisplay {
                name: project.name,
                key: project.key,
                id: project.id,
            };
            projects_display.push(project_display);
        }
        projects_display
    }
}
