#![feature(associated_type_defaults)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

mod fields;
mod file_utilities;
mod jira_structs;
mod project;
mod traits;

extern crate base64;
extern crate pretty_env_logger;
extern crate rpassword;

use crate::fields::CustomFieldsHandler;
use crate::jira_structs::{JiraMeta};
use crate::project::ProjectHandler;
use crate::traits::Searchable;
use reqwest::Client;
use std::env;
use structopt::StructOpt;

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

    let custom_fields_handler = CustomFieldsHandler {
        jira_meta: jira_meta.clone(),
        custom_fields: None,
    };

    let project_handler = ProjectHandler {
        jira_meta: jira_meta.clone(),
    };

    let project = &project_handler.list(&REST_CLIENT).await;
    println!("Project: {:?}", project[0]);

    let custom_fields = &custom_fields_handler.list(&REST_CLIENT).await;

    let epic: &Vec<String> = custom_fields.get("Epic Link").unwrap();
    println!("CUSTOM FIELDS {:?}", epic[0]);

    // let yaml_path = opt.input;
    //
    // // Load Yaml file.
    // let yaml = load_yaml(&yaml_path).await;
    // build_req(&jira_api, &user, &pass, &yaml).await.unwrap();
}
