use serde::{Deserialize, Serialize};

pub mod client;
pub mod scheduler;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitAuthor {
    name: String,
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    id: String,
    message: String,
    author: CommitAuthor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub full_name: String,
    pub url: String,
    pub clone_url: String,

    #[serde(skip_deserializing)]
    pub stages: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trigger {
    #[serde(rename = "ref")]
    pub branch: String,

    pub head_commit: Commit,
    pub repository: Project,
}
