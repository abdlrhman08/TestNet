pub mod interface {
    tonic::include_proto!("mainservice");
}

use std::collections::HashMap;
use std::sync::mpsc::Sender;
// we may need the tokio mutex, let's see how will this go
use std::sync::{Arc, Mutex};
use std::time::Duration;

use interface::test_net_server::TestNet;
use interface::{Empty, Job, Sentinel};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
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

pub struct NodeData {
    sender: JobSender,
    busy_state: bool,
}

pub type JobSender = Sender<Result<Job, Status>>;
pub type NodeMap = Arc<Mutex<HashMap<String, NodeData>>>;

pub struct TestNetServer {
    nodes: NodeMap,
    job_queue: Arc<Mutex<JobQueue>>,
}

impl TestNetServer {
    pub fn new(job_queue: Arc<Mutex<JobQueue>>, nodes: NodeMap) -> Self {
        Self { job_queue, nodes }
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

    type registerStream = ReceiverStream<Result<Job, Status>>;

    async fn register(&self, _: Request<Empty>) -> Result<Response<Self::registerStream>, Status> {
        let (mut tx, rx) = mpsc::channel(1);

        tokio::task::spawn(async move {
            let mut t = 0;
            loop {
                t += 1;
                let job = Job {
                    payload: format!("Hello from {}", t),
                };
                tx.send(Ok(job)).await.unwrap();
                std::thread::sleep(Duration::from_secs(2));
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
