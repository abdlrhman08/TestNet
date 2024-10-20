pub mod interface {
    tonic::include_proto!("mainservice");
}

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, LazyLock};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

use axum::extract::ws::{Message, WebSocket};

use serde::Serialize;

use interface::test_net_server::TestNet;
use interface::{
    Empty, Job, LogObject, Stages
};

use tokio::sync::mpsc::{self, Sender};
//TODO!: explain the usage of tokio mutex
use tokio::sync::Mutex;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

// I hate what i am gonna do
pub static LOG_STREAMER: LazyLock<Mutex<Option<WebSocket>>> = LazyLock::new(|| { Mutex::new(None) });

#[derive(Serialize)]
struct SerializableLog {
    project: u64,
    stage: String,
    log: String,

    // is preferred to mke this a u8, but for simplicity we'll leave it as i32
    exit_code: Option<i32>,
}

fn generate_random_id() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

pub struct JobQueue(VecDeque<Job>);

impl JobQueue {
    pub fn new() -> Self {
        JobQueue(VecDeque::new())
    }

    pub fn queue_job(&mut self, job: Job) {
        self.0.push_back(job)
    }

    pub fn get_upcoming(&mut self) -> Option<Job> {
        self.0.pop_front()
    }
}

pub type JobSender = Sender<Result<Job, Status>>;
// i'll leave the nodes data as a hashmap for now, because we can still
// iterate through (key, value) pairs. But if we used a vector from
// now, we may have to reimplement everything later
pub type NodeMap = Arc<Mutex<HashMap<String, NodeData>>>;

// set public for now
#[derive(Debug)]
pub struct NodeData {
    pub id: String,
    sender: JobSender,
    pub busy_state: bool,
}

impl NodeData {
    pub fn get_sender(&self) -> &JobSender {
        &(self.sender)
    }
}

pub struct TestNetServer {
    nodes: NodeMap,
    job_queue: Arc<Mutex<JobQueue>>,
}

impl TestNetServer {
    pub fn new(job_queue: &Arc<Mutex<JobQueue>>, nodes: &NodeMap) -> Self {
        let cloned_queue = Arc::clone(job_queue);
        let cloned_nodemap = Arc::clone(&nodes);
        Self {
            job_queue: cloned_queue,
            nodes: cloned_nodemap,
        }
    }
}

#[tonic::async_trait]
impl TestNet for TestNetServer {
    async fn pull_job(&self, _: Request<Empty>) -> Result<Response<Job>, Status> {
        // i need help
        let job_queue = &mut *self.job_queue.lock().await;
        let upcoming_job = job_queue.get_upcoming().unwrap();
        Ok(Response::new(upcoming_job))
    }

    async fn register_stages(&self, stages: Request<Stages>) -> Result<Response<Empty>, Status> {
        println!("Resolved stages {:?}", stages);
        Ok(Response::new(Empty {}))
    }

    async fn send_log(&self, log: Request<LogObject>) -> Result<Response<Empty>, Status> {
        //TODO!: stream the logs through web sockets
        let LogObject { job_id, stage, log, status_code } = log.into_inner();
        let log = SerializableLog { 
            project: job_id, 
            stage, log,
            exit_code: status_code
        };
        let json_log = serde_json::to_string(&log).unwrap();

        if let Some(ws) = &mut *LOG_STREAMER.lock().await {
            let _sock = ws.send(Message::Text(json_log)).await;
        } else {
            todo!();
        }

        Ok(Response::new(Empty {}))
    }

    type registerStream = ReceiverStream<Result<Job, Status>>;

    async fn register(&self, _: Request<Empty>) -> Result<Response<Self::registerStream>, Status> {
        let (tx, rx) = mpsc::channel(1);
        let id = generate_random_id();
        let node_data = NodeData {
            id: id.clone(),
            sender: tx,
            busy_state: false,
        };

        let node_map = &mut *self.nodes.lock().await;
        node_map.insert(id, node_data);

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
