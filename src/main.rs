#![feature(associated_type_defaults)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

mod fields;
mod jira_structs;
mod traits;
mod file_utilities;

extern crate base64;
extern crate pretty_env_logger;
extern crate rpassword;
extern crate serde_json;

use crate::fields::CustomFieldsHandler;
use crate::jira_structs::{Component, Fields, FieldsMeta, IssueType, IssuesResponse, Project, JiraMeta};
use crate::traits::Searchable;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::Read;
use structopt::StructOpt;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Stories input file
    #[structopt(short = "s", long = "stories")]
    input: Option<String>,

    /// Jira user
    #[structopt(short = "u", long = "user")]
    user: String,

    /// Jira Host
    #[structopt(short = "h", long = "host")]
    host: String,

    /// Log Level
    #[structopt(short = "l", long = "log", default_value = "INFO")]
    log_level: String,
}

lazy_static! {
    static ref REST_CLIENT: Client = reqwest::Client::new();
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    env::set_var("RUST_LOG", opt.log_level.to_ascii_uppercase());
    pretty_env_logger::init();

    let pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
    let user = opt.user;

    // let jira_api = format!("{}{}", opt.host.to_string(), "/rest/api/2");
    let jira_meta = JiraMeta {
        host: opt.host.to_string(),
        user,
        pass,
    };

    let custom_fields_handler = CustomFieldsHandler{
        jira_meta
    };

    let custom_fields =
        &custom_fields_handler.list(&REST_CLIENT).await.unwrap();

    println!("CUSTOM FIELDS {:?}", custom_fields);

    // let yaml_path = opt.input;
    //
    // // Load Yaml file.
    // let yaml = load_yaml(&yaml_path).await;
    // build_req(&jira_api, &user, &pass, &yaml).await.unwrap();
}

async fn build_req(
    jira_api_path: &str,
    user: &str,
    pass: &str,
    yaml: &Yaml,
) -> Result<(), Box<dyn std::error::Error>> {
    let story_uri = format!("{}{}", jira_api_path.clone(), "/issue/");

    // Build req body.
    let req_bodies = build_req_body(user, pass, jira_api_path, &yaml).await;
    for b in req_bodies.clone() {
        debug!("BODY {}", b.get("fields").unwrap());
    }
    for b in req_bodies {
        let body = serde_json::to_string(&b).unwrap();

        let res = REST_CLIENT
            .post(&story_uri)
            .basic_auth(user, Some(&pass))
            .header(CONTENT_TYPE, "application/json")
            .body(body.clone())
            .send()
            .await?;

        if res.status().is_success() {
            info!(
                "Historia {:#?} criada com sucesso",
                b.get("fields").unwrap().get("summary")
            );
        } else {
            error!(
                "Nao foi possivel criar historia: {:#?}",
                b.get("fields").unwrap().get("summary")
            );
            debug!("{:#?}", res);
        }
    }
    Ok(())
}

async fn build_req_body(
    user: &str,
    pass: &str,
    jira_api: &str,
    yaml: &Yaml,
) -> Vec<serde_json::Value> {
    let fields_meta = build_req_meta(&yaml);

    let existent_stories: Vec<String> =
        get_existent_stories(user, pass, jira_api, &fields_meta.customfield_10101)
            .await
            .unwrap();

    let mut req_list: Vec<serde_json::Value> = Vec::new();

    // Get yaml block "stories".
    for story in yaml["stories"].as_vec().unwrap() {
        let summary: String = story["name"].as_str().unwrap().to_string();

        if existent_stories.contains(&summary) {
            error!("'Historia {}' -> Já existe.", summary);
        } else {
            let description: String = story["description"].as_str().unwrap().to_string();

            let req_body = json!({
                "fields": {
                    "summary": summary,
                    "description": description,
                    "project": fields_meta.project,
                    "issuetype": fields_meta.issuetype,
                    "labels": fields_meta.labels,
                    "components": fields_meta.components,
                    "customfield_10101": fields_meta.customfield_10101,
                    "customfield_10214": fields_meta.customfield_10214,
                },
            });
            debug!("Req body: {:?}", req_body);
            req_list.push(req_body.clone());
        };
    }
    req_list
}

async fn get_existent_stories(
    user: &str,
    pass: &str,
    jira_api_path: &str,
    epic: &str,
) -> Result<Vec<String>, reqwest::Error> {
    let epic_custom_field: i32 = 10101 as i32;
    let search_uri = format!(
        "{}/search?jql=cf[{}]={}",
        jira_api_path, epic_custom_field, epic
    );

    debug!("Search uri {:?}", search_uri);

    let issues = REST_CLIENT
        .get(&search_uri)
        .basic_auth(user, Some(pass))
        .send()
        .await?
        .json::<IssuesResponse>()
        .await;

    let mut epic_issues: Vec<String> = Vec::new();

    for issue in issues.unwrap().issues {
        let parsed_issue = issue
            .get("fields")
            .unwrap()
            .get("summary")
            .unwrap()
            .to_string()
            .replace("\"", "")
            .replace("\\n", "");
        epic_issues.push(parsed_issue);
    }

    Ok(epic_issues)
}

fn build_req_meta(yaml: &Yaml) -> FieldsMeta {
    let yaml_metadata = &yaml["metadata"];
    let epic = yaml_metadata.clone()["epic"].as_str().unwrap().to_string();
    let team = yaml_metadata.clone()["team"].as_str().unwrap().to_string();

    let issuetype = IssueType {
        name: "Story".to_string(),
    };

    let mut labels: Vec<String> = Vec::new();
    for label in yaml_metadata.clone()["labels"].as_vec().unwrap() {
        labels.push(label.as_str().unwrap().to_string());
    }

    let mut components: Vec<Component> = Vec::new();
    for component in yaml_metadata.clone()["components"].as_vec().unwrap() {
        components.push(Component {
            name: component.as_str().unwrap().to_string(),
        });
    }

    let project_key = yaml_metadata.clone()["project"]
        .as_str()
        .unwrap()
        .to_string();

    let project = Project { key: project_key };

    FieldsMeta {
        issuetype,
        labels,
        components,
        project,
        customfield_10101: epic.to_string(),
        customfield_10214: team.to_string(),
    }
}
