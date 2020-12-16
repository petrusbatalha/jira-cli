#![feature(associated_type_defaults)]
#![feature(default_free_fn)]
#![feature(option_insert)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate json_patch;

use serde_json::json;

mod epic;
mod fields;
mod file_utilities;
mod jira_structs;
mod project;
mod stories;
mod traits;

extern crate base64;
extern crate pretty_env_logger;

use crate::epic::EpicHandler;
use crate::fields::CustomFieldsHandler;
use crate::project::ProjectHandler;
use crate::stories::{StoriesHandler, Story};
use crate::traits::{ArgOptions, Searchable};
use reqwest::Client;
use std::default::default;
use std::env;
use structopt::StructOpt;
use yaml_rust::YamlLoader;
use std::collections::HashMap;

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

    let conf_string = file_utilities::load_yaml(&CONF_PATH).await;
    let conf = &YamlLoader::load_from_str(&conf_string.unwrap()).unwrap()[0];

    let arg_option = ArgOptions {
        project,
        epic: Some("ESTRT-1293".to_string()),
        host: conf["jira"]["host"].as_str().unwrap().to_owned(),
        user: Some(conf["jira"]["user"].as_str().unwrap().to_owned()),
        pass: Some(conf["jira"]["pass"].as_str().unwrap().to_owned()),
    };

    // &ProjectHandler.list(&arg_option, &REST_CLIENT).await;
    // &CustomFieldsHandler
    //     .cache_custom_fields(&arg_option, &REST_CLIENT)
    //     .await;
    let mut map_team = HashMap::new();
    &map_team.insert("Team".to_string(), "timitisson".to_string());
    let mut map_link = HashMap::new();
    &map_link.insert("Epic Link".to_string(), "Estrututut".to_string());

    &StoriesHandler.list(&arg_option, &REST_CLIENT).await;
    let story = &StoriesHandler
        .create_story(
            None,
            Some("TITULO".to_string()),
            Some("Descricao".to_string()),
            None,
            None,
            // Some(vec![map_link, map_team]),
            None,
        )
        .await;
    println!("STORYYY {:?}", story);

    // let epic_link: &Vec<String> = custom_fields.get("Epic Link").unwrap();
    // println!("CUSTOM FIELDS {:?}", epic_link[0]);

    // let epic = &EpicHandler.list(&arg_option, &REST_CLIENT).await;
    // println!("EPIC {:?}", epic.as_ref());
}
