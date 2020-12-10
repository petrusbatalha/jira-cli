use crate::jira_structs::{REST_URI, Issue, JQL};
use crate::traits::{Searchable, ArgOptions};
use async_trait::async_trait;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::Deserialize;
use anyhow::bail;
use url::Url;
pub struct EpicHandler;

#[derive(Debug, Clone, Deserialize)]
pub struct Epic {
    pub expand: Option<String>,
    #[serde(rename = "startAt")]
    pub start_at: Option<i32>,
    #[serde(rename = "maxResults")]
    pub max_result: Option<i32>,
    pub total: Option<i32>,
    pub issues: Option<Vec<Issue>>,
}

// Query para listar epicos no jira.
// https://jira.bradesco.com.br:8443/rest/api/2/search?jql=PROJECT=ESTRT AND issuetype="Epic"&fields=summary

#[async_trait]
impl Searchable<Result<Epic, anyhow::Error>> for EpicHandler {
    async fn list(&self, options: &ArgOptions, client: &Client) -> Result<Epic,  anyhow::Error> {
        let uri = format!("{}{}", &options.host, &REST_URI);

        let jql_query = match &options.project {
            Some(p) => {
                format!("{}{}{}{}{}", &uri, &JQL, "PROJECT=", p,
                        " AND issuetype=Epic&fields=summary,description")
            },
            _ => {
                bail!("Please, define the project to list epics".to_string())
            }
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

        Ok(epics)
    }
}
