use crate::jira_structs::{Issue, JQL, REST_URI};
use crate::traits::{ArgOptions, Searchable};
use anyhow::bail;
use async_trait::async_trait;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::Deserialize;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{Table, TableStyle};
use url::Url;

pub struct EpicHandler;

#[derive(Debug, Clone, Deserialize)]
pub struct Epic {
    pub issues: Option<Vec<Issue>>,
}

// Query para listar epicos no jira.
// https://jira.bradesco.com.br:8443/rest/api/2/search?jql=PROJECT=ESTRT AND issuetype="Epic"&fields=summary

#[async_trait]
impl Searchable<Result<(), anyhow::Error>> for EpicHandler {
    async fn list(&self, options: &ArgOptions, client: &Client) -> Result<(), anyhow::Error> {
        let uri = format!("{}{}", &options.host, &REST_URI);

        let jql_query = match &options.project {
            Some(p) => format!(
                "{}{}{}{}{}",
                &uri, &JQL, "PROJECT=", p, " AND issuetype=Epic&fields=summary,description"
            ),
            _ => bail!("Please, define the project to list epics".to_string()),
        };

        let epics = client
            .get(Url::parse(&jql_query).unwrap())
            .basic_auth(&options.user.as_ref().unwrap(), options.clone().pass)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .unwrap()
            .json::<Epic>()
            .await
            .unwrap();

        let mut table = Table::new();
        table.max_column_width = 80;
        table.style = TableStyle::blank();
        table.add_row(build_table_header_row());

        for issue in epics.issues.unwrap() {
            table.add_row(build_table_body(issue));
        }

        println!("{}", table.render());

        Ok(())
    }
}

fn build_table_body(issue: Issue) -> Row<'static> {
    let fields = issue.fields.unwrap();
    Row::new(vec![
        TableCell::new_with_alignment(fields.summary.unwrap(), 1, Alignment::Left),
        TableCell::new_with_alignment(issue.key, 2, Alignment::Left),
        TableCell::new_with_alignment(issue.id, 1, Alignment::Left),
        TableCell::new_with_alignment(issue.issue_link, 1, Alignment::Left),
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
