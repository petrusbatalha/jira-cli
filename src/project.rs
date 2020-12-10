use crate::jira_structs::{REST_URI};
use crate::traits::{Searchable, ArgOptions};
use async_trait::async_trait;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::Deserialize;
use std::ops;
use core::fmt;

static PROJECT_URI: &str = "/project";

pub struct ProjectHandler;

pub struct ProjectsDisplay<'a>(pub &'a Vec<ProjectDisplay>);

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

impl<'a> fmt::Display for ProjectsDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter().fold(Ok(()), |result, project| {
            result.and_then(|_| writeln!(f, "|Name|\t\t\t|Key|\n{}\t{}", project.name, project.key))
        })
    }
}

impl<'a> ops::Deref for ProjectsDisplay<'a> {
    type Target = Vec<ProjectDisplay>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl Searchable<Result<(), ()>> for ProjectHandler {
    async fn list(&self, options: &ArgOptions, client: &Client) -> Result<(), ()> {
        let uri = format!("{}{}{}", &options.host, &REST_URI, &PROJECT_URI);

        let projects = client
            .get(&uri)
            .basic_auth(&options.user.as_ref().unwrap(), options.clone().pass)
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

        info!("{}", ProjectsDisplay {0: &projects_display,});

        Ok(())
    }
}
