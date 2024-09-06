pub mod interface {
    tonic::include_proto!("mainservice");
}

// we may need the tokio mutex, let's see how will this go
use std::sync::{Arc, Mutex};

use interface::test_net_server::TestNet;
use interface::{Job, Sentinel};
use tonic::{Request, Response, Status};

pub struct JobQueue(Vec<Job>);

impl JobQueue {
    pub fn new() -> Self {
        JobQueue(vec![])
    }

    pub fn queue_job(&mut self, job: Job) {
        self.0.push(job);
    }

    pub fn get_upcoming(&mut self) -> Job {
        self.0.remove(0)
    }
}

pub struct TestNetServer {
    job_queue: Arc<Mutex<JobQueue>>,
}

impl TestNetServer {
    pub fn new(job_queue: Arc<Mutex<JobQueue>>) -> Self {
        Self { job_queue }
    }
}

#[tonic::async_trait]
impl TestNet for TestNetServer {
    async fn pull_job(&self, _: Request<Sentinel>) -> Result<Response<Job>, Status> {
        // i need help
        let job_queue = &mut *self.job_queue.lock().unwrap();
        let upcoming_job = job_queue.get_upcoming();
        Ok(Response::new(upcoming_job))
    }
}
