#![feature(associated_type_defaults)]
#![feature(default_free_fn)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

mod epic;
mod fields;
mod file_utilities;
mod jira_structs;
mod project;
mod traits;

extern crate base64;
extern crate pretty_env_logger;

use crate::epic::EpicHandler;
use crate::fields::CustomFieldsHandler;
use crate::project::ProjectHandler;
use crate::traits::{ArgOptions, Searchable};
use reqwest::Client;
use std::env;
use structopt::StructOpt;

const CONF_PATH: &str = "./.conf.yaml";

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
    user: Option<String>,

    /// Jira Host
    #[structopt(short = "h", long = "host")]
    host: Option<String>,

    // Jira Project
    #[structopt(short = "p", long = "project")]
    jira_project: Option<String>,

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
    let project = opt.jira_project;

    env::set_var("RUST_LOG", opt.log_level.to_ascii_uppercase());
    pretty_env_logger::init();

    let conf = file_utilities::load_yaml(&CONF_PATH).await;

    let arg_option = ArgOptions {
        project,
        host: conf["jira"]["host"].as_str().unwrap().to_owned(),
        user: Some(conf["jira"]["user"].as_str().unwrap().to_owned()),
        pass: Some(conf["jira"]["pass"].as_str().unwrap().to_owned()),
    };

    // &ProjectHandler.list(&arg_option, &REST_CLIENT).await;
    &CustomFieldsHandler
        .cache_custom_fields(&arg_option, &REST_CLIENT)
        .await;
    println!(
        "EPICO LINK {}",
        &CustomFieldsHandler
            .get_custom_field("Epic Link")
            .await
            .unwrap()
    );

    // let epic_link: &Vec<String> = custom_fields.get("Epic Link").unwrap();
    // println!("CUSTOM FIELDS {:?}", epic_link[0]);

    // let epic = &EpicHandler.list(&arg_option, &REST_CLIENT).await;
    // println!("EPIC {:?}", epic.as_ref());
}
