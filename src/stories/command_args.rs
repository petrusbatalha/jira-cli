use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct StoryOps {
    #[structopt(long = "project", short = "p", help = "Project to create stories")]
    pub project: Option<String>,
    #[structopt(long = "epic", short = "e", help = "Epic to link stories")]
    pub epic: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    #[structopt(
        long = "template",
        short = "t",
        help = "Link to template for creating stories"
    )]
    pub template_path: Option<String>,
    #[structopt(long = "file", short = "f", help = "Stories yaml file.")]
    pub file: String,
}

#[derive(StructOpt, Debug)]
pub struct StoryListOps {
    #[structopt(long = "epic", short = "e", help = "Epic to list stories for.")]
    pub epic: String,
    #[structopt(
        long = "project",
        short = "p",
        help = "Project wich contain the epics."
    )]
    pub project: String,
}
