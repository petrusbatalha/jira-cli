#![feature(associated_type_defaults)]
#![feature(default_free_fn)]
#![feature(option_insert)]
#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;

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
use crate::stories::{StoriesHandler};
use crate::traits::{ArgOptions, Searchable};
use reqwest::Client;
use std::env;
use structopt::StructOpt;
use yaml_rust::YamlLoader;

const CONF_PATH: &str = "./.jira-cli/conf.yaml";

#[derive(Debug, StructOpt)]
#[structopt(name = "jira-cli")]
pub struct Opts {
    /// Log Level
    #[structopt(short = "l", long = "log", default_value = "INFO")]
    log_level: String,

    /// SUBCOMMANDS
    #[structopt(subcommand)]
    commands: Option<Commands>,
}

#[derive(StructOpt, Debug)]
enum Commands {
    /// List jira issue types.
    #[structopt(name = "list")]
    List(List),
    #[structopt(name = "add")]
    Add(Add),
}

#[derive(StructOpt, Debug)]
enum Add {
    #[structopt(name = "story")]
    Story(StoryOps),
}

#[derive(StructOpt, Debug)]
enum List {
    #[structopt(name = "projects")]
    Project(ProjectOps),
    #[structopt(name = "stories")]
    Story(StoryOps),
    #[structopt(name = "epics")]
    Epic(EpicOps),
}

#[derive(StructOpt, Debug)]
pub struct ProjectOps {}

#[derive(StructOpt, Debug)]
pub struct EpicOps {
    #[structopt(long = "project", short = "p")]
    project_key: String,
}

#[derive(StructOpt, Debug)]
pub struct StoryOps {
    #[structopt(long = "project", short = "p")]
    project: Option<String>,
    #[structopt(long = "epic", short = "e")]
    epic: Option<String>,
    #[structopt(long = "summary", short = "s")]
    summary: Option<String>,
    #[structopt(long = "description", short = "d")]
    description: Option<String>,
    #[structopt(long = "labels", short = "l")]
    labels: Option<Vec<String>>,
    #[structopt(long = "customfields", short = "c")]
    custom_fields: Option<String>,
    #[structopt(long = "template", short = "t")]
    template_path: Option<String>,
}

lazy_static! {
    static ref REST_CLIENT: Client = reqwest::Client::new();
}

#[tokio::main]
async fn main() {
    let opts = Opts::from_args();

    env::set_var("RUST_LOG", opts.log_level.to_ascii_uppercase());
    pretty_env_logger::init();

    let conf_string = file_utilities::load_yaml(&CONF_PATH).await;
    let conf = &YamlLoader::load_from_str(&conf_string.unwrap()).unwrap()[0];

    let arg_options = ArgOptions {
        host: conf["jira"]["host"].as_str().unwrap().to_owned(),
        user: Some(conf["jira"]["user"].as_str().unwrap().to_owned()),
        pass: Some(conf["jira"]["pass"].as_str().unwrap().to_owned()),
    };

    &CustomFieldsHandler
        .cache_custom_fields(&arg_options, &REST_CLIENT)
        .await;

    handle_args(opts, &arg_options).await;
}

async fn handle_args(opts: Opts, fixed_options: &ArgOptions) {
    if let Some(subcommand) = opts.commands {
        match subcommand {
            Commands::List(issue_type) => match issue_type {
                List::Story(args) => {
                    &StoriesHandler
                        .list(&args, fixed_options, &REST_CLIENT).await;
                }
                List::Epic(args) => {
                    &EpicHandler.list(&args, fixed_options, &REST_CLIENT).await;
                }
                List::Project(args) => {
                    &ProjectHandler.list(&args, fixed_options, &REST_CLIENT).await;
                }
            },
            Commands::Add(issue_type) => match issue_type {
                Add::Story(args) => {
                    &StoriesHandler.create_story(args).await;
                }
            },
        }
    }
}