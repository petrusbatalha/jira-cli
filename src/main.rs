extern crate base64;
extern crate pretty_env_logger;
extern crate rpassword;
extern crate serde_json;
extern crate yaml_rust;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
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
    input: String,

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

#[derive(Debug, Clone, Serialize)]
struct FieldsMeta {
    issuetype: IssueType,
    labels: Vec<String>,
    components: Vec<Component>,
    project: Project,
    customfield_10214: String,
    customfield_10101: String,
}

#[derive(Debug, Clone, Serialize)]
struct Fields {
    summary: String,
    description: String,
}

#[derive(Debug, Clone, Serialize)]
struct Project {
    key: String,
}

#[derive(Debug, Clone, Serialize)]
struct IssueType {
    name: String,
}

#[derive(Debug, Clone, Serialize)]
struct Component {
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct IssuesResponse {
    issues: Vec<Value>,
}

lazy_static! {
    static ref REST_CLIENT: Client = reqwest::Client::new();
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    env::set_var("RUST_LOG", opt.log_level.to_ascii_uppercase());
    pretty_env_logger::init();

    let jira_api = format!("{}{}", opt.host.to_string(), "/rest/api/2");
    let pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
    let user = opt.user;
    let yaml_path = opt.input;

    // Load Yaml file.
    let yaml = load_yaml(&yaml_path).await;
    build_req(&jira_api, &user, &pass, &yaml).await.unwrap();
}

async fn load_yaml(yaml_path: &str) -> Yaml {
    // Open stories yaml.
    let mut stories_file = File::open(yaml_path).unwrap();
    let mut yaml_as_string = String::new();
    stories_file
        .read_to_string(&mut yaml_as_string)
        .expect("Failed to load yaml");

    let yaml_file = YamlLoader::load_from_str(&yaml_as_string).unwrap();
    yaml_file[0].clone()
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
