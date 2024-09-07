use serde::{Deserialize, Serialize};

pub mod client;
pub mod scheduler;

#[derive(Debug, Serialize, Deserialize)]
pub struct Trigger;
