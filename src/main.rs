#![feature(associated_type_defaults)]
#![feature(default_free_fn)]
#![feature(option_insert)]
#[macro_use]
extern crate log;

mod commons;
mod issues;
mod stories;

extern crate base64;
extern crate pretty_env_logger;

use stories::command_args::{StoryListOps, StoryOps};
use crate::issues::{epic::EpicHandler, project::ProjectHandler};
use crate::stories::stories::StoriesHandler;
use commons::{
    file_utilities::load_yaml,
    structs::AuthOptions,
    traits::Searchable,
};
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

    handle_args(opts, &auth_options).await;
}

async fn handle_args(opts: Opts, auth_options: &AuthOptions) {
    if let Some(subcommand) = opts.commands {
        match subcommand {
            Commands::List(issue_type) => match issue_type {
                List::Story(args) => {
                    StoriesHandler.list(&args, auth_options).await;
                }
                List::Epic(args) => {
                    EpicHandler.list(&args, auth_options).await;
                }
                List::Project(args) => {
                    ProjectHandler.list(&args, auth_options).await;
                }
            },
            Commands::Add(issue_type) => match issue_type {
                Add::Story(args) => {
                    StoriesHandler.create_story(&args, auth_options).await;
                }
            },
        }
    }
}
