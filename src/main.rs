extern crate rpassword;
extern crate base64;
extern crate serde_json;
extern crate yaml_rust;
use yaml_rust::{YamlLoader, Yaml};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use std::fs::File;
use reqwest::header::{HeaderMap, HeaderValue};
use std::io::Read;

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
}

#[derive(Debug, Serialize, Deserialize)]
struct IssueBody {
    fields: Fields,
}

#[derive(Debug, Serialize, Deserialize)]
struct Fields {
    project: Project,
    summary: String,
    description: String,
    issuetype: IssueType,
    labels: Vec<String>,
    components: Vec<String>,
    customfield_10214: String,
    customfield_10101: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct IssueType {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   let opt = Opt::from_args();
    let story_uri = opt.host.to_string() + "/rest/api/2/issue/";

    // Build auth headers.
    let pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
    let credentials = "Basic ".to_string() + &*base64::encode(opt.user + ":" + &pass);

    // Open stories yaml.
    let mut stories_file = File::open(opt.input).unwrap();
    let mut yaml_as_string = String::new();
    stories_file.read_to_string(&mut yaml_as_string).expect("Failed to load yaml");

    let yaml_file = YamlLoader::load_from_str(&yaml_as_string).unwrap();
    let yaml = &yaml_file[0];

    // Build req body.
    let req_bodies = build_req_body(yaml.clone());

    let client = reqwest::Client::new();

    for body in req_bodies {
        // let res =
        //     client.post(&story_uri)
        //         .headers(get_auth(&credentials))
        //         .body(body)
        //         .send()
        //         .await?;
        println!("{:?}", body);
    }

    Ok(())
}

fn get_auth(credentials: &str) -> HeaderMap {
    let content_type: String = "application/json".to_string();
    let mut headers = HeaderMap::new();

    headers.insert("Authorization",
                   HeaderValue::from_str(credentials).unwrap());
    headers.insert("Content-Type",
                   HeaderValue::from_str(&content_type).unwrap());
    headers
}

fn build_req_body(yaml: Yaml) -> Vec<String> {
    let yaml_metadata = &yaml["metadata"];
    let epic = yaml_metadata.clone()["epic"].as_str().unwrap().to_string();
    let team = yaml_metadata.clone()["team"].as_str().unwrap().to_string();
    let project = yaml_metadata.clone()["project"].as_str().unwrap().to_string();
    let mut req_list: Vec<String> = Vec::new();

    let mut labels: Vec<String> = Vec::new();
    for label in yaml_metadata.clone()["labels"].as_vec().unwrap() {
        labels.push(label.as_str().unwrap().to_string());
    };

    let mut components: Vec<String> = Vec::new();
    for component in yaml_metadata.clone()["components"].as_vec().unwrap() {
        components.push(component.as_str().unwrap().to_string());
    };

    // Get yaml block "stories".
    for story in yaml["stories"].as_vec().unwrap() {
        let labels = labels.clone();
        let components = components.clone();
        let key = project.clone();
        let customfield_10101= epic.clone();
        let customfield_10214= team.clone();

        // Get nested fields
        // Get project key.
        let project: Project = Project { key };

        // Get Issue Type.
        let issuetype = IssueType { name: "Story".to_string()};

        // Get single fields
        let summary: String = story.clone()["name"].as_str().unwrap().to_string();
        let description: String = story.clone()["description"].as_str().unwrap().to_string();

        let req_body =
            IssueBody { fields: Fields {
                project,
                summary,
                description,
                issuetype,
                labels,
                components,
                customfield_10101,
                customfield_10214,
            }
        };

        let body = serde_json::to_string(&req_body).unwrap();

        req_list.push(body.clone());

        println!("Serialized: {}", body);
    }
    req_list
}