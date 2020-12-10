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
mod epic;

extern crate base64;
extern crate pretty_env_logger;
extern crate rpassword;

use crate::fields::CustomFieldsHandler;
use crate::project::ProjectHandler;
use crate::traits::{Searchable, ArgOptions};
use structopt::StructOpt;
use reqwest::Client;
use std::env;
use crate::epic::EpicHandler;

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

    // Jira Project
    #[structopt(short = "p", long = "project", default_value = "")]
    jira_project: String,

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

    let p = opt.jira_project;

    let project = if p.is_empty() {
        None
    } else {
        Some(p)
    };

    let arg_option = ArgOptions {
        project,
        host: opt.host.to_string(),
        user: Some(user),
        pass: Some(pass),
    };

    let project = &ProjectHandler.list(&arg_option, &REST_CLIENT).await;
    println!("Project: {:?}", project);

    let custom_fields = &CustomFieldsHandler.list(&arg_option,&REST_CLIENT).await;

    let epic_link: &Vec<String> = custom_fields.get("Epic Link").unwrap();
    println!("CUSTOM FIELDS {:?}", epic_link[0]);

    let epic = &EpicHandler.list(&arg_option, &REST_CLIENT).await;
    println!("CUSTOM FIELDS {:?}", epic.as_ref());


    // let yaml_path = opt.input;
    //
    // // Load Yaml file.
    // let yaml = load_yaml(&yaml_path).await;
    // build_req(&jira_api, &user, &pass, &yaml).await.unwrap();
}
