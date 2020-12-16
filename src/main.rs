#![feature(associated_type_defaults)]
#![feature(default_free_fn)]
#![feature(option_insert)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

mod commons;
mod issues;

extern crate base64;
extern crate pretty_env_logger;

use commons::{traits::Searchable,file_utilities::load_yaml, structs::AuthOptions, custom_fields::{CustomFieldsCache, CustomFieldsHandler}};
use crate::issues::{project::ProjectHandler, epic::EpicHandler, stories::StoriesHandler};
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
    /// List jira objects [Projects, Stories and Epics]
    #[structopt(name = "list")]
    List(List),
    /// Add jira objects, currently supported [Stories]
    #[structopt(name = "add")]
    Add(Add),
}

#[derive(StructOpt, Debug)]
enum Add {
    /// Create jira stories, see [add stories --help] for more
    #[structopt(name = "stories")]
    Story(StoryOps),
}

#[derive(StructOpt, Debug)]
enum List {
    #[structopt(name = "projects")]
    Project(ProjectOps),
    #[structopt(name = "stories")]
    Story(StoryListOps),
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
    #[structopt(long = "project", short = "p", help = "Project to create stories")]
    project: Option<String>,
    #[structopt(long = "epic", short = "e", help = "Epic to link stories")]
    epic: Option<String>,
    #[structopt(long = "summary", short = "s", help = "Story summary")]
    summary: Option<String>,
    #[structopt(long = "description", short = "d", help = "Story Description")]
    description: Option<String>,
    #[structopt(long = "labels", short = "l", help = "Story Labels")]
    labels: Option<Vec<String>>,
    #[structopt(
        long = "template",
        short = "t",
        help = "Link to template for creating stories"
    )]
    template_path: Option<String>,
}

#[derive(StructOpt, Debug)]
pub struct StoryListOps {
    #[structopt(long = "epic", short = "e", help = "Epic to list stories for.")]
    epic: Option<String>,
}

lazy_static! {
    static ref REST_CLIENT: Client = reqwest::Client::new();
}

#[tokio::main]
async fn main() {
    let opts = Opts::from_args();

    env::set_var("RUST_LOG", opts.log_level.to_ascii_uppercase());
    pretty_env_logger::init();

    let conf_string = load_yaml(&CONF_PATH).await.unwrap();
    let conf = &YamlLoader::load_from_str(&conf_string).unwrap()[0];
    let auth_options = AuthOptions {
        host: conf["jira"]["host"].as_str().unwrap().to_owned(),
        user: Some(conf["jira"]["user"].as_str().unwrap().to_owned()),
        pass: Some(conf["jira"]["pass"].as_str().unwrap().to_owned()),
    };

    let custom_fields_cache = CustomFieldsHandler
        .cache_custom_fields(&auth_options, &REST_CLIENT)
        .await
        .unwrap();

    &handle_args(opts, custom_fields_cache, &auth_options).await;
}

async fn handle_args(
    opts: Opts,
    custom_fields_cache: CustomFieldsCache,
    auth_options: &AuthOptions,
) {
    if let Some(subcommand) = opts.commands {
        match subcommand {
            Commands::List(issue_type) => match issue_type {
                List::Story(args) => {
                    &StoriesHandler
                        .list(&args, auth_options, &custom_fields_cache, &REST_CLIENT)
                        .await;
                }
                List::Epic(args) => {
                    &EpicHandler
                        .list(&args, auth_options, &custom_fields_cache, &REST_CLIENT)
                        .await;
                }
                List::Project(args) => {
                    &ProjectHandler
                        .list(&args, auth_options, &custom_fields_cache, &REST_CLIENT)
                        .await;
                }
            },
            Commands::Add(issue_type) => match issue_type {
                Add::Story(args) => {
                    &StoriesHandler
                        .create_story(args, auth_options, &custom_fields_cache)
                        .await;
                }
            },
        }
    }
}
