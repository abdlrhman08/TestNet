use serde::{Deserialize, Serialize};

pub mod client;
pub mod scheduler;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CommitAuthor {
    name: String,
    username: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Commit {
    id: String,
    message: String,
    author: CommitAuthor,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub url: String,
    pub clone_url: String,

    #[serde(skip_deserializing)]
    pub stages: Option<Vec<String>>
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Trigger {
    #[serde(default)]
    #[serde(rename = "ref")]
    pub branch: String,

    #[serde(default)]
    pub head_commit: Commit,

    #[serde(default)]
    pub hook_id: u64,

    #[serde(default)]
    pub repository: Project,
}

#[derive(Serialize, Deserialize)]
pub struct Notification {
    // just added so the front-end can distinguish from normal data
    pub notification: bool,
    pub data: String
}
