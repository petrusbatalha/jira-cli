use crate::commons::structs::Issue;
use serde::Deserialize;

pub struct EpicHandler;

#[derive(Debug, Clone, Deserialize)]
pub struct Epic {
    pub issues: Option<Vec<Issue>>,
}
