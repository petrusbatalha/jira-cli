use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct EpicOps {
    #[structopt(long = "project", short = "p")]
    pub(crate) project_key: String,
}
