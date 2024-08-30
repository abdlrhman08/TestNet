pub mod interface {
    tonic::include_proto!("mainservice");
}

use interface::test_net_server::TestNet;
use interface::{Job, Sentinel};
use tonic::{Request, Response, Status};

pub struct TestNetServer {
    job_queue: Vec<Job>,
}

impl TestNetServer {
    pub fn new() -> Self {
        Self { job_queue: vec![] }
    }
}

#[tonic::async_trait]
impl TestNet for TestNetServer {
    async fn pull_job(&self, request: Request<Sentinel>) -> Result<Response<Job>, Status> {
        let job = Job {
            payload: "i  pulled job from the queue".to_string(),
        };

        Ok(Response::new(job))
    }
}
