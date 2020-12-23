use serde::Deserialize;
use crate::commons::structs::Issue;

pub struct EpicHandler;

#[derive(Debug, Clone, Deserialize)]
pub struct Epic {
    pub issues: Option<Vec<Issue>>,
}
