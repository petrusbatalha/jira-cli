use crate::jira_structs::{REST_URI, Issue, JQL};
use crate::traits::{ArgOptions, Searchable};
use async_trait::async_trait;
use core::fmt;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::Deserialize;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{Table, TableStyle};
use crate::fields::CustomFieldsHandler;

static STORIES_URI: &str = "/project";

pub struct StoriesHandler;

#[derive(Debug, Clone, Deserialize)]
pub struct Stories {
    pub expand: Option<String>,
    #[serde(rename = "startAt")]
    pub start_at: Option<i32>,
    #[serde(rename = "maxResults")]
    pub max_result: Option<i32>,
    pub total: Option<i32>,
    pub issues: Vec<Issue>,
}

// let search_uri = format!(
//      "{}/search?jql=cf[{}]={}",
//      jira_api_path, epic_custom_field, epic
//  );
//

#[async_trait]
impl Searchable<Result<(), ()>> for StoriesHandler {
    async fn list(&self, options: &ArgOptions, client: &Client) -> Result<(), ()> {
        let uri = format!("{}{}", &options.host, &REST_URI);

        let epic_uri = format!(
                "{}{}{}{}{}",
                &uri, &JQL,  &CustomFieldsHandler
                    .get_custom_field("Epic Link")
                    .await
                    .unwrap(), "=", &options.epic.as_ref().unwrap());

        let stories = client
            .get(&epic_uri)
            .basic_auth(&options.user.as_ref().unwrap(), options.clone().pass)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .unwrap()
            .json::<Stories>()
            .await
            .unwrap();

        let mut table = Table::new();
        table.max_column_width = 80;
        table.style = TableStyle::blank();

        table.add_row(build_table_header_row());

        for issue in  stories.issues {
            table.add_row(build_table_body(issue));
        }

        println!("{}", table.render());

        Ok(())
    }
}

fn build_table_body(stories: Issue) -> Row<'static> {
    Row::new(vec![
        TableCell::new_with_alignment(stories.fields.unwrap().summary.unwrap(), 1, Alignment::Left),
        TableCell::new_with_alignment(stories.key, 1, Alignment::Left),
        TableCell::new_with_alignment(stories.id, 2, Alignment::Left),
        TableCell::new_with_alignment(stories.issue_link, 1, Alignment::Left),
    ])
}

fn build_table_header_row() -> Row<'static> {
    Row::new(vec![
        TableCell::new_with_alignment("Name", 1, Alignment::Left),
        TableCell::new_with_alignment("Key", 2, Alignment::Left),
        TableCell::new_with_alignment("ID", 1, Alignment::Left),
        TableCell::new_with_alignment("Link", 1, Alignment::Left),
    ])
}

