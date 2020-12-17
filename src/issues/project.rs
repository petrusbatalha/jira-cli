use crate::commons::structs::{AuthOptions, REST_URI};
use crate::commons::traits::Searchable;
use crate::commons::custom_fields::CustomFieldsCache;
use crate::{ProjectOps};
use async_trait::async_trait;
use reqwest::{header::CONTENT_TYPE, Client};
use serde::Deserialize;
use std::fmt;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};
use reqwest::Url;
use crate::commons::req_builder::build_req;

static PROJECT_URI: &str = "/project";

pub struct ProjectHandler;

#[derive(Debug, Clone)]
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

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} \t\t {} \t {} \t", self.key, self.name, self.id)
    }
}

#[async_trait]
impl Searchable<ProjectOps> for ProjectHandler {
    async fn list(
        &self,
        _options: &ProjectOps,
        auth_options: &AuthOptions,
        _custom_fields_cache: &CustomFieldsCache,
    ) {
        let url =
            Url::parse(&format!("{}{}{}", &auth_options.host, &REST_URI, &PROJECT_URI)).unwrap();

        debug!("Listing projects... will call uri: {}", url.clone());

        let projects =
            build_req(url, auth_options)
            .send()
            .await
            .unwrap()
            .json::<Vec<Project>>()
            .await
            .unwrap();

        let mut table = Table::new();
        table.max_column_width = 40;
        table.style = TableStyle::blank();

        table.add_row(build_table_header_row());

        for project in projects.clone() {
            table.add_row(build_table_body(project));
        }

        println!("{}", table.render());
    }
}

fn build_table_body(project: Project) -> Row<'static> {
    Row::new(vec![
        TableCell::new_with_alignment(project.key, 1, Alignment::Left),
        TableCell::new_with_alignment(project.name, 2, Alignment::Left),
        TableCell::new_with_alignment(project.id, 1, Alignment::Left),
    ])
}

fn build_table_header_row() -> Row<'static> {
    Row::new(vec![
        TableCell::new_with_alignment("Key", 1, Alignment::Left),
        TableCell::new_with_alignment("Name", 2, Alignment::Left),
        TableCell::new_with_alignment("ID", 1, Alignment::Left),
    ])
}
