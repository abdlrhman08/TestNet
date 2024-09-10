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

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    id: i32,
    name: String,
    full_name: String,
    url: String,
    git_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trigger {
    #[serde(rename = "ref")]
    pub branch: String,

    pub head_commit: Commit,
    pub repository: Project,
}
