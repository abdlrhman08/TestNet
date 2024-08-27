use serde::Deserialize;

pub struct Job {}

#[derive(Debug, Deserialize)]
pub struct Trigger {
    trigger_type: String,
}

impl Trigger {
    fn new() -> Self {
        Self {
            trigger_type: "Test".to_string(),
        }
    }
}

pub struct JobQueue(Vec<Job>);

impl JobQueue {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push_job(job: Job) {
        // Will notify the nodes of a new job addition or either the nodes
        // will query when they are ready
        unimplemented!();
    }
}
